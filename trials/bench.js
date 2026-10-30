// The T-02 performance script, the same file in every variant (docs/STEPS.md, T-02).
//
// It plays each scenario as real DOM events and times every step from the input to the frame
// that shows its result: it waits, through a MutationObserver, for the step's visible change,
// then for the next animation frame and the task after it, when that frame has been painted.
//
// The page it drives has these hooks, the same in every variant:
//   #diagram (data-ready, data-rev)    the whole-building diagram; data-rev rises on each reload
//   #diagram-frame                     takes drags and wheel events
//   #diagram-view (data-x)             carries the pan and zoom; data-x is the rounded pan x
//   #part-<id> (data-x)                a part; data-x is its rounded x position
//   #edit-address                      re-addresses the edited part
//   #findings (data-rev)               the findings list; data-rev rises on each check run
//   #next-page, #sheet (data-page)     the page view
//   #overview-toggle, #overview (data-count)   every page at once
//   #filter, #rows (data-rev)          the long list
//   #reload                            replaces the project, as an outside write does
(function () {
  "use strict";

  const STEP_TIMEOUT_MS = 30000;
  const OPEN_TIMEOUT_MS = 180000;
  const HUB = 0;

  const $ = (selector) => document.querySelector(selector);
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

  // Resolves after the next frame has been painted.
  const afterPaint = () =>
    new Promise((resolve) => requestAnimationFrame(() => setTimeout(() => resolve(performance.now()), 0)));

  // Resolves when `test()` holds, checking on every DOM change under `root`.
  function until(test, root, timeoutMs) {
    return new Promise((resolve, reject) => {
      if (test()) return resolve(performance.now());
      const observer = new MutationObserver(() => {
        if (test()) {
          observer.disconnect();
          clearTimeout(timer);
          resolve(performance.now());
        }
      });
      observer.observe(root || document.documentElement, { attributes: true, childList: true, subtree: true, characterData: true });
      const timer = setTimeout(() => {
        observer.disconnect();
        reject(new Error("timed out"));
      }, timeoutMs || STEP_TIMEOUT_MS);
    });
  }

  // Times one step: from just before `act()` to the painted frame after `done()` holds.
  async function step(act, done, root) {
    const start = performance.now();
    act();
    await until(done, root);
    return (await afterPaint()) - start;
  }

  // Records frame intervals while `work` runs.
  async function frames(work) {
    const stamps = [];
    let running = true;
    const tick = (t) => {
      stamps.push(t);
      if (running) requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
    await work();
    running = false;
    return stamps.slice(1).map((t, i) => t - stamps[i]);
  }

  function summary(values) {
    const clean = values.filter((v) => typeof v === "number" && isFinite(v)).sort((a, b) => a - b);
    if (!clean.length) return { n: 0 };
    const at = (q) => clean[Math.min(clean.length - 1, Math.floor(q * clean.length))];
    const round = (v) => Math.round(v * 10) / 10;
    return {
      n: clean.length,
      p50: round(at(0.5)),
      p95: round(at(0.95)),
      max: round(clean[clean.length - 1]),
      mean: round(clean.reduce((a, b) => a + b, 0) / clean.length),
    };
  }

  function frameSummary(intervals) {
    return Object.assign(summary(intervals), { dropped: intervals.filter((v) => v > 25).length });
  }

  const pointer = (type, target, x, y) =>
    target.dispatchEvent(
      new PointerEvent(type, { bubbles: true, cancelable: true, composed: true, pointerId: 1, isPrimary: true, button: 0, buttons: type === "pointerup" ? 0 : 1, clientX: x, clientY: y }),
    );
  const centre = (el) => {
    const r = el.getBoundingClientRect();
    return [r.left + r.width / 2, r.top + r.height / 2];
  };
  function type(input, value) {
    input.value = value;
    input.dispatchEvent(new Event("input", { bubbles: true }));
  }
  const rev = (selector) => Number(($(selector) || {}).dataset?.rev || 0);
  const view = () => $("#diagram-view").getAttribute("transform") + "|" + $("#diagram-view").getAttribute("style");

  const scenarios = {
    // Drag the hub part 3 px a step, 120 steps at 60 Hz, as a pointer would. Steps overlap, so
    // each one waits for its own expected position.
    async drag(config) {
      const steps = config.capacity ? 60 : 120;
      const part = $("#part-" + HUB);
      const frame = $("#diagram-frame");
      const startX = Number(part.dataset.x);
      let [x, y] = centre(part);
      pointer("pointerdown", part, x, y);
      const pending = [];
      const intervals = await frames(async () => {
        for (let k = 1; k <= steps; k++) {
          x += 3;
          const expected = startX + 3 * k;
          pending.push(step(() => pointer("pointermove", frame, x, y), () => Number(part.dataset.x) >= expected, part).catch(() => NaN));
          await sleep(16);
        }
        await Promise.all(pending);
      });
      pointer("pointerup", frame, x, y);
      const all = await Promise.all(pending);
      return { latency: summary(all), frames: frameSummary(intervals), timeouts: all.filter((v) => !isFinite(v)).length };
    },

    // Pan the whole diagram by dragging its background, 2 px a step, 120 steps at 60 Hz.
    async pan() {
      const frame = $("#diagram-frame");
      const shown = () => Number($("#diagram-view").dataset.x);
      const r = frame.getBoundingClientRect();
      let x = r.left + 5;
      const y = r.top + 5;
      const startX = shown();
      pointer("pointerdown", frame, x, y);
      const pending = [];
      const intervals = await frames(async () => {
        for (let k = 1; k <= 120; k++) {
          x += 2;
          const expected = startX + 2 * k;
          pending.push(step(() => pointer("pointermove", frame, x, y), () => shown() >= expected, $("#diagram-view")).catch(() => NaN));
          await sleep(16);
        }
        await Promise.all(pending);
      });
      pointer("pointerup", frame, x, y);
      const all = await Promise.all(pending);
      return { latency: summary(all), frames: frameSummary(intervals), timeouts: all.filter((v) => !isFinite(v)).length };
    },

    // Zoom in and out with the wheel, 20 steps.
    async zoom() {
      const frame = $("#diagram-frame");
      const [x, y] = centre(frame);
      const latencies = [];
      for (let k = 0; k < 20; k++) {
        const before = view();
        const wheel = new WheelEvent("wheel", { bubbles: true, cancelable: true, deltaY: k % 2 ? 100 : -100, clientX: x, clientY: y });
        latencies.push(await step(() => frame.dispatchEvent(wheel), () => view() !== before, $("#diagram-view")).catch(() => NaN));
      }
      return { latency: summary(latencies) };
    },

    // Re-address one part 20 times; every check reruns and the findings update.
    async edit(config) {
      const input = $("#edit-address");
      const latencies = [];
      for (let k = 0; k < (config.capacity ? 5 : 20); k++) {
        const before = rev("#findings");
        latencies.push(await step(() => type(input, String(k % 2 ? 200 : 3)), () => rev("#findings") > before, $("#findings")).catch(() => NaN));
      }
      return { latency: summary(latencies), findings: Number($("#findings").dataset.count || 0) };
    },

    // Flip through 20 pages.
    async pages() {
      const latencies = [];
      for (let k = 0; k < 20; k++) {
        const next = Number($("#sheet").dataset.page) + 1;
        latencies.push(await step(() => $("#next-page").click(), () => Number($("#sheet").dataset.page) === next, $("#sheet")).catch(() => NaN));
      }
      return { latency: summary(latencies) };
    },

    // Show every page at once, then hide them again.
    async overview(config) {
      const ms = await step(() => $("#overview-toggle").click(), () => Number(($("#overview") || {}).dataset?.count) === config.pages).catch(() => NaN);
      $("#overview-toggle").click();
      await until(() => !$("#overview")).catch(() => {});
      await afterPaint();
      return { ms: Math.round(ms) };
    },

    // Type five characters into the long list's filter.
    async list() {
      const input = $("#filter");
      const latencies = [];
      let text = "";
      for (const ch of "w-001") {
        text += ch;
        const before = rev("#rows");
        latencies.push(await step(() => type(input, text), () => rev("#rows") > before, $("#rows")).catch(() => NaN));
      }
      const before = rev("#rows");
      type(input, "");
      await until(() => rev("#rows") > before).catch(() => {});
      return { latency: summary(latencies) };
    },

    // Replace the whole project, as an outside write does, 3 times.
    async reload() {
      const latencies = [];
      for (let k = 0; k < 3; k++) {
        const before = rev("#diagram");
        latencies.push(await step(() => $("#reload").click(), () => rev("#diagram") > before && $("#diagram").dataset.ready === "1", $("#diagram"), OPEN_TIMEOUT_MS).catch(() => NaN));
      }
      return { latency: summary(latencies) };
    },
  };

  // Runs every scenario once and returns the results. `config` carries the variant, the size,
  // the page count, and the process start time in epoch ms (for the open time). With `capacity`,
  // only opening, dragging and editing are timed, and opening gets 30 seconds.
  window.hyleonBench = async function (config) {
    const results = { variant: config.variant, size: config.size, slowMs: config.slowMs || 0, capacity: !!config.capacity, userAgent: navigator.userAgent, errors: [] };
    try {
      await until(() => $("#diagram") && $("#diagram").dataset.ready === "1", null, config.capacity ? 30000 : OPEN_TIMEOUT_MS);
      await afterPaint();
      results.open = { ms: Math.round(Date.now() - config.startEpochMs), pageMs: Math.round(performance.now()) };
    } catch (e) {
      results.errors.push("open: " + e.message);
      return results;
    }
    const names = config.capacity ? ["drag", "edit"] : ["drag", "pan", "zoom", "edit", "pages", "overview", "list", "reload"];
    for (const name of names) {
      try {
        await sleep(300);
        results[name] = await scenarios[name](config);
      } catch (e) {
        results.errors.push(name + ": " + e.message);
      }
    }
    return results;
  };
})();
