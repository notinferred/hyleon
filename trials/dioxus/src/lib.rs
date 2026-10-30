//! Variant D of the T-02 comparison: the end-case screen in Dioxus 0.7, with the core native and in
//! the same process. State is fine-grained, as Dioxus recommends: one signal per part position,
//! and memoized parts and wires, so a drag re-renders only what moved.
//!
//! In bench mode it plays `trials/bench.js` and hands back the results.

use bench_core::{Command, EDITED, Finding, Project, Severity, Size};
use dioxus::prelude::*;

const BENCH_JS: &str = include_str!("../../bench.js");
const BENCH_CSS: &str = include_str!("../../bench.css");
const PART_WIDTH: f32 = 80.0;
const PART_HEIGHT: f32 = 24.0;
const LIST_LIMIT: usize = 500;

/// How the app was started. Desktop knows at launch; the web build reads its address first.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub size: Size,
    pub bench: bool,
    pub slow_ms: u64,
    /// Where desktop writes the results; the web build posts them to `/results` instead.
    pub out: Option<String>,
    /// The process start, in epoch milliseconds. The web build uses the page's own start.
    pub start_epoch_ms: Option<f64>,
}

impl Settings {
    /// Reads `?size=building&bench=1&slow=50` from the web build's address.
    pub fn from_query(query: &str) -> Settings {
        let value = |key: &str| {
            query
                .trim_start_matches('?')
                .split('&')
                .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
        };
        Settings {
            size: value("size").and_then(Size::parse).unwrap_or(Size::Panel),
            bench: value("bench") == Some("1"),
            slow_ms: value("slow").and_then(|s| s.parse().ok()).unwrap_or(0),
            out: None,
            start_epoch_ms: None,
        }
    }
}

/// Given at launch: desktop fills it in, the web build leaves it empty.
#[derive(Clone)]
pub struct Launch(pub Option<Settings>);

#[derive(Clone, Copy, PartialEq)]
struct View {
    x: f32,
    y: f32,
    zoom: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum Gesture {
    Drag {
        part: u32,
        from: (f64, f64),
        start: (f32, f32),
    },
    Pan {
        from: (f64, f64),
        start: (f32, f32),
    },
}

/// The screen's state. The core is read with `peek`, so only these signals drive rendering.
#[derive(Clone, Copy)]
struct Model {
    core: Signal<Project>,
    positions: Signal<Vec<Signal<(f32, f32)>>>,
    diagram_rev: Signal<u32>,
    findings: Signal<Vec<Finding>>,
    findings_rev: Signal<u32>,
    page: Signal<u32>,
    overview: Signal<bool>,
    filter: Signal<String>,
    rows_rev: Signal<u32>,
    view: Signal<View>,
    gesture: Signal<Option<Gesture>>,
    slow_ms: u64,
}

impl Model {
    fn new(size: Size, slow_ms: u64) -> Model {
        let project = Project::generate(size, 1);
        let findings = project.check();
        Model {
            positions: Signal::new(positions(&project)),
            core: Signal::new(project),
            diagram_rev: Signal::new(0),
            findings: Signal::new(findings),
            findings_rev: Signal::new(0),
            page: Signal::new(0),
            overview: Signal::new(false),
            filter: Signal::new(String::new()),
            rows_rev: Signal::new(0),
            view: Signal::new(View {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            }),
            gesture: Signal::new(None),
            slow_ms,
        }
    }

    /// Reruns every check, as Hyleon does on every change. `--slow` adds a busy wait first, so
    /// the harness can prove it measures what it should. Only desktop has the clock for it.
    fn recheck(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if self.slow_ms > 0 {
            let until = std::time::Instant::now() + std::time::Duration::from_millis(self.slow_ms);
            while std::time::Instant::now() < until {}
        }
        #[cfg(target_arch = "wasm32")]
        let _ = self.slow_ms;
        let findings = self.core.peek().check();
        self.findings.set(findings);
        self.findings_rev += 1;
    }
}

fn positions(project: &Project) -> Vec<Signal<(f32, f32)>> {
    project
        .parts
        .iter()
        .map(|p| Signal::new((p.x, p.y)))
        .collect()
}

fn severity_class(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

#[component]
pub fn App() -> Element {
    let launch = use_context::<Launch>();
    let mut settings = use_signal(|| launch.0.clone());
    use_future(move || async move {
        if settings.peek().is_none() {
            let query = document::eval("return location.search")
                .await
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default();
            settings.set(Some(Settings::from_query(&query)));
        }
    });
    match settings() {
        Some(settings) => rsx! { Screen { settings } },
        None => rsx! { p { "Loading…" } },
    }
}

#[component]
fn Screen(settings: Settings) -> Element {
    let model = use_context_provider(|| Model::new(settings.size, settings.slow_ms));
    let bench = settings.clone();
    use_effect(move || {
        if bench.bench {
            spawn(run_bench(bench.clone(), model));
        }
    });

    rsx! {
        style { {BENCH_CSS} }
        div { class: "bench",
            Controls {}
            Diagram {}
            div { class: "side",
                Findings {}
                List {}
            }
            Pages {}
        }
    }
}

async fn run_bench(settings: Settings, model: Model) {
    let config = serde_json::json!({
        "variant": "D",
        "size": settings.size.name(),
        "pages": model.core.peek().pages,
        "slowMs": settings.slow_ms,
        "startEpochMs": settings.start_epoch_ms,
        "post": settings.out.is_none(),
    });
    let script = format!(
        "{BENCH_JS}\nconst config = {config};\nif (config.startEpochMs === null) config.startEpochMs = performance.timeOrigin;\nconst results = await window.hyleonBench(config);\nif (config.post) await fetch('/results', {{ method: 'POST', body: JSON.stringify(results) }});\nreturn results;"
    );
    let results = document::eval(&script).await;
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(out) = settings.out {
        let text = match results {
            Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_default(),
            Err(e) => format!("{{\"errors\": [\"eval: {e}\"]}}"),
        };
        if let Err(e) = std::fs::write(&out, text) {
            eprintln!("couldn't write {out}: {e}");
        }
        std::process::exit(0);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = results;
}

#[component]
fn Controls() -> Element {
    let mut model = use_context::<Model>();
    rsx! {
        div { class: "controls",
            label {
                "Address of P-0002"
                input {
                    id: "edit-address",
                    r#type: "number",
                    oninput: move |evt| {
                        if let Ok(address) = evt.value().parse::<u8>() {
                            model.core.write().apply(Command::SetAddress { part: EDITED, address });
                            model.recheck();
                        }
                    },
                }
            }
            button { id: "next-page", onclick: move |_| model.page += 1, "Next page" }
            button {
                id: "overview-toggle",
                onclick: move |_| model.overview.toggle(),
                "All pages"
            }
            label {
                "Filter"
                input {
                    id: "filter",
                    oninput: move |evt| {
                        model.filter.set(evt.value());
                        model.rows_rev += 1;
                    },
                }
            }
            button {
                id: "reload",
                onclick: move |_| {
                    // An outside write: the other generated project, read back from bytes.
                    let size = model.core.peek().size;
                    let seed = if model.diagram_rev.peek().is_multiple_of(2) { 2 } else { 1 };
                    let bytes = Project::generate(size, seed).to_bytes();
                    if let Ok(project) = Project::from_bytes(&bytes) {
                        model.positions.set(positions(&project));
                        model.core.set(project);
                        model.recheck();
                        model.rows_rev += 1;
                        model.diagram_rev += 1;
                    }
                },
                "Reload"
            }
        }
    }
}

#[component]
fn Diagram() -> Element {
    let mut model = use_context::<Model>();
    let View { x, y, zoom } = (model.view)();
    let rev = (model.diagram_rev)();
    let pages = model.core.peek().pages as f32;
    let tiles = pages.sqrt().ceil();
    let (width, height) = (tiles * 1000.0, tiles * 700.0);
    let shown_x = x.round() as i64;

    rsx! {
        div {
            id: "diagram-frame",
            onpointerdown: move |evt| {
                let at = evt.client_coordinates();
                let view = *model.view.peek();
                model.gesture.set(Some(Gesture::Pan { from: (at.x, at.y), start: (view.x, view.y) }));
            },
            onpointermove: move |evt| {
                let Some(gesture) = *model.gesture.peek() else { return };
                let at = evt.client_coordinates();
                match gesture {
                    Gesture::Drag { part, from, start } => {
                        let zoom = model.view.peek().zoom;
                        let x = start.0 + (at.x - from.0) as f32 / zoom;
                        let y = start.1 + (at.y - from.1) as f32 / zoom;
                        model.core.write().apply(Command::Move { part, x, y });
                        let mut position = model.positions.peek()[part as usize];
                        position.set((x, y));
                    }
                    Gesture::Pan { from, start } => {
                        let mut view = model.view.write();
                        view.x = start.0 + (at.x - from.0) as f32;
                        view.y = start.1 + (at.y - from.1) as f32;
                    }
                }
            },
            onpointerup: move |_| model.gesture.set(None),
            onwheel: move |evt| {
                evt.prevent_default();
                let factor = if evt.delta().strip_units().y < 0.0 { 1.1 } else { 1.0 / 1.1 };
                model.view.write().zoom *= factor;
            },
            svg {
                id: "diagram",
                "data-ready": "1",
                "data-rev": "{rev}",
                width: "{width}",
                height: "{height}",
                g {
                    id: "diagram-view",
                    "data-x": "{shown_x}",
                    transform: "translate({x} {y}) scale({zoom})",
                    DiagramContent {}
                }
            }
        }
    }
}

/// Every wire and part. It takes no props, so panning and zooming never re-render it.
#[component]
fn DiagramContent() -> Element {
    let model = use_context::<Model>();
    let _rev = (model.diagram_rev)();
    let positions = model.positions.read();
    let core = model.core.peek();

    rsx! {
        g {
            for c in core.connections.iter() {
                Wire {
                    key: "{c.id}",
                    from: positions[c.from as usize],
                    to: positions[c.to as usize],
                }
            }
        }
        g {
            for p in core.parts.iter() {
                PartView {
                    key: "{p.id}",
                    id: p.id,
                    tag: p.tag.clone(),
                    at: positions[p.id as usize],
                }
            }
        }
    }
}

#[component]
fn Wire(from: Signal<(f32, f32)>, to: Signal<(f32, f32)>) -> Element {
    let (x1, y1) = from();
    let (x2, y2) = to();
    let (x1, y1, x2, y2) = (
        x1 + PART_WIDTH / 2.0,
        y1 + PART_HEIGHT / 2.0,
        x2 + PART_WIDTH / 2.0,
        y2 + PART_HEIGHT / 2.0,
    );
    rsx! {
        line { class: "wire", x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}" }
    }
}

#[component]
fn PartView(id: u32, tag: String, at: Signal<(f32, f32)>) -> Element {
    let mut model = use_context::<Model>();
    let (x, y) = at();
    let shown_x = x.round() as i64;
    rsx! {
        g {
            id: "part-{id}",
            class: "part",
            "data-x": "{shown_x}",
            transform: "translate({x} {y})",
            onpointerdown: move |evt| {
                evt.stop_propagation();
                let from = evt.client_coordinates();
                model.gesture.set(Some(Gesture::Drag { part: id, from: (from.x, from.y), start: at() }));
            },
            rect { width: "{PART_WIDTH}", height: "{PART_HEIGHT}", rx: "3" }
            text { x: "6", y: "16", "{tag}" }
        }
    }
}

#[component]
fn Findings() -> Element {
    let model = use_context::<Model>();
    let rev = (model.findings_rev)();
    let findings = model.findings.read();
    let count = findings.len();
    rsx! {
        div { id: "findings", "data-rev": "{rev}", "data-count": "{count}",
            h2 { "Findings ({count})" }
            ul {
                for f in findings.iter() {
                    li {
                        key: "{f.id}",
                        class: severity_class(f.severity),
                        "{f.what} {f.place}"
                    }
                }
            }
        }
    }
}

#[component]
fn List() -> Element {
    let model = use_context::<Model>();
    let rev = (model.rows_rev)();
    let filter = (model.filter)();
    let (rows, total) = model.core.peek().rows(&filter, LIST_LIMIT);
    rsx! {
        div { id: "rows", "data-rev": "{rev}",
            h2 { "{total} parts and connections" }
            table {
                tbody {
                    for r in rows {
                        tr { key: "{r.key}",
                            td { "{r.tag}" }
                            td { "{r.kind}" }
                            td { "{r.page}" }
                            td { "{r.detail}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Pages() -> Element {
    let model = use_context::<Model>();
    let page = (model.page)();
    let overview = (model.overview)();
    let _rev = (model.diagram_rev)();
    let core = model.core.peek();
    let pages = core.pages;
    let shown = (page % pages as u32) as u16;
    rsx! {
        div { class: "pages",
            if overview {
                div { id: "overview", "data-count": "{pages}",
                    for p in 0..pages {
                        div { key: "{p}", class: "thumb", dangerous_inner_html: core.sheet_svg(p) }
                    }
                }
            } else {
                div { id: "sheet", "data-page": "{page}", dangerous_inner_html: core.sheet_svg(shown) }
            }
        }
    }
}
