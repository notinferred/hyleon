# Appendix 1: CI at scale for a Rust monorepo (agent report, 2026-10-29)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-29-delivery.md](../2026-10-29-delivery.md).

**The brief:** how large projects keep CI fast and cheap as the codebase grows, and what fits a Rust monorepo like Hyleon: compilation caching, building and testing only what changed, keeping main green without pull requests, tiered CI in real Rust projects, compile-time settings that matter in CI, and CI time budgets.

---

# CI at scale for a Rust monorepo: what large projects do, and what fits Hyleon

Research date: 2026-10-29. Sources are linked inline. "Measured" means I pulled the run data myself from the GitHub REST API on 2026-10-29; the method is under Limits. Vendor claims are labelled as such.

## Key findings

- **Big Rust apps still use plain Cargo on GitHub Actions.** Zed, Ruff, uv, Bevy, Tauri and Deno all do. Their PR CI takes about 7–65 minutes wall-clock (measured, see section 4). They keep it there with five levers: faster runners, dependency caching, running only what changed, splitting checks into tiers, and hard timeouts. None of them uses Bazel.
- **Bazel and Buck2 show up only in very large or multi-language codebases.** Examples are Meta, DFINITY (about 600k lines of Rust) and OpenAI Codex (experimental). The teams that report on it describe real tooling and maintenance costs.
- **Hyleon's plan limits the options.** It is a private repo on a GitHub Free organization. That means:
  - CI machines have 2 vCPU and 8 GB RAM.
  - The cache is capped at 10 GB, and Free can't buy more.
  - There is no merge queue, no branch protection and no rulesets.
  - So "main always passes" can only be kept by habit, or by upgrading the plan.
- **At about 40 s, one job is still the cheapest correct setup.** GitHub rounds each job up to a whole minute, so splitting into parallel jobs raises billed minutes.

## 1. Compilation caching

**Swatinem/rust-cache** ([README](https://github.com/Swatinem/rust-cache)):
- Caches `~/.cargo` and the dependency artifacts in `target/`. It deliberately skips workspace crates ("generally not effective").
- Sets `CARGO_INCREMENTAL=0` and deletes artifacts older than a week.
- Keys the cache on the rustc version and the Cargo.lock/Cargo.toml hashes. On a miss it falls back to the cache from a previous lockfile.
- `save-if` limits which runs write the cache; Ruff, uv and Tauri write only from their main branch.

**sccache** caches each compiler call separately. Backends include local disk, S3, R2, GCS, Azure, Redis, WebDAV and the GitHub Actions cache ([README](https://github.com/mozilla/sccache)). Its Rust caveats ([Rust.md](https://github.com/mozilla/sccache/blob/main/docs/Rust.md)):
- Incremental compilation must be off.
- Crates that call the linker (bin, dylib, cdylib, proc-macro) are never cached.
- Absolute paths must match between machines unless `SCCACHE_BASEDIRS` is set.
- For Cloudflare R2 you must set `SCCACHE_ENDPOINT` and `SCCACHE_REGION=auto` ([S3.md](https://github.com/mozilla/sccache/blob/main/docs/S3.md)).

**How the two interact.** They complement each other:
- rust-cache restores one archive. Per cache key it is all or nothing, and fast when it hits.
- sccache works per crate. It still hits after a Cargo.lock bump, and it shares results across jobs and branches.
- If both use the GitHub cache, they compete for the same 10 GB.
- Zed runs both: Namespace cache volumes, plus sccache stored in R2 (`R2_ACCOUNT_ID`, `SCCACHE_BUCKET: sccache-zed`) ([run_tests.yml](https://github.com/zed-industries/zed/blob/main/.github/workflows/run_tests.yml)).
- rust-lang/rust uses sccache on S3, mainly for LLVM ([rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/tests/ci.html)).

**Public measurements.** None of these is a rigorous benchmark:
- **neosmart (2024-07-10), native Linux:** a clean rebuild with a warm sccache took 14.12 s versus 1 m 10 s without it (about 80% faster). A cold cache was up to about 8% slower. On WSL, sccache made builds slower overall ([post](https://neosmart.net/blog/benchmarking-rust-compilation-speedups-and-slowdowns-from-sccache-and-zthreads/)).
- **WezTerm switched from rust-cache to sccache** (PR merged 2024-02-01). macOS went "45min vs 56min". The author called it "not a big improvement but it helps with cold caches" ([PR #4468](https://github.com/wez/wezterm/pull/4468)).
- **xuanwo (2023-04):** up to about 50% faster than rust-cache on maturin as the hit rate rose. Some jobs were slower ([report](https://xuanwo.io/en-us/reports/2023-04/)).
- **Depot (vendor claim):** 35% faster with nextest and a warm sccache on 8-core runners. Depot also warns that the GitHub cache backend can return HTTP 429 under load, which sccache treats as misses ([blog](https://depot.dev/blog/sccache-in-github-actions)).

**GitHub cache limits in 2026:**
- **Size and eviction:** 10 GB per repository is included. Entries not accessed for 7 days are deleted, and the least recently used go first when the cap is hit ([docs](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)).
- **Paying for more:** since 2025-11-20 a repo can exceed 10 GB, but only on Pro, Team or Enterprise ([changelog](https://github.blog/changelog/2025-11-20-github-actions-cache-size-can-now-exceed-10-gb-per-repository/)). The extra costs $0.07 per GB-month ([billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions)).
- **Upload rate:** since 2026-01-16, each repo may upload at most 200 cache entries per minute ([changelog](https://github.blog/changelog/2026-01-16-rate-limiting-for-actions-cache-entries/)). Downloads are capped at 1,500 per minute ([limits](https://docs.github.com/en/actions/reference/limits)). This matters for sccache's GitHub backend, which writes one entry per crate.
- **Scope:** a run can restore caches from its own branch or the default branch. Since 2026-06-26, runs started by untrusted triggers, including `workflow_run`, can only read the default branch's cache ([changelog](https://github.blog/changelog/2026-06-26-read-only-actions-cache-for-untrusted-triggers/)).
- **R2 as an alternative:** the free tier is 10 GB-month of storage, 1M Class A and 10M Class B operations per month, with no egress fees ([pricing](https://developers.cloudflare.com/r2/pricing/)).

**Pitfall: timestamps.** Cargo decides what to rebuild partly from file modification times, and a fresh checkout resets them. So cached artifacts for workspace crates get rebuilt anyway. Deno maintains its own "mtime cache" action to work around this ([action](https://github.com/denoland/deno/tree/main/.github/mtime_cache)).

## 2. Running only what changed, and monorepo build systems

**guppy determinator** ([docs](https://docs.rs/determinator), [README](https://github.com/guppy-rs/guppy/blob/main/tools/determinator/README.md)):
- Works out which packages changed between two commits, from the changed files plus simulated Cargo builds.
- It can't run build scripts and "may produce both false negatives and false positives".
- The authors recommend using it on PRs and doing "full runs every few hours on the main branch".
- It was built for the Diem Core repo.

**Zed's lighter version** ([run_tests.yml](https://github.com/zed-industries/zed/blob/main/.github/workflows/run_tests.yml)):
- Maps changed directories to packages using `cargo metadata`.
- Builds a nextest filter `rdeps(pkg)|…`. `rdeps` means the crate plus everything that depends on it, directly or indirectly ([nextest](https://nexte.st/docs/filtersets/reference/)).
- Any change to root Cargo files, the toolchain, `.cargo/` or `.github/` triggers a full run.
- Pushes to main always run the full suite.

**cargo-hakari** generates a "workspace-hack" crate so that shared dependencies are built once with a single feature set:
- Its docs claim "up to 100x" faster for some local builds and "up to 1.7x" overall, without naming a benchmark ([docs](https://docs.rs/cargo-hakari)).
- Oxide's omicron and nextest itself use it (both have `workspace-hack/`).
- Zed adopted it in April 2025 and removed it on 2025-10-17 ([commit](https://github.com/zed-industries/zed/commit/ef5b8c6fed25da85658ad2d376323f26daa88296)):
  - Without it, the number of crates built rose from about 1,700 to about 2,000.
  - They removed it anyway because of spurious rebuilds and friction with automated dependency updates.
  - They expect cargo#14774 to bring the same benefit natively.
- Cargo's native setting, `resolver.feature-unification = "workspace"`, still requires nightly ([Cargo guide](https://doc.rust-lang.org/cargo/guide/build-performance.html)).

**nextest sharding:**
- `--partition slice:m/n` or `hash:m/n` splits tests across machines.
- `cargo nextest archive` builds once and runs the tests on many machines, as long as they use the same revision and nextest version ([partitioning](https://nexte.st/docs/ci-features/partitioning/), [archiving](https://nexte.st/docs/ci-features/archiving/)).
- uv splits its Windows tests three ways with `hash:k/3` ([test.yml](https://github.com/astral-sh/uv/blob/main/.github/workflows/test.yml)).

**nextest flaky-test handling** ([retries](https://nexte.st/docs/features/retries/)):
- `retries` reruns failed tests, optionally with backoff.
- `flaky-result = "fail"` (version 0.9.131+) still fails the run but labels the test as flaky.
- Omicron's CI profile uses `retries = 2` with `flaky-result = "fail"` ([config](https://github.com/oxidecomputer/omicron/blob/main/.config/nextest.toml)).
- Zed caps each test's run time with `slow-timeout … terminate-after` ([config](https://github.com/zed-industries/zed/blob/main/.config/nextest.toml)).

**Monorepo build systems:**
- **Bazel with rules_rust:**
  - DFINITY (about 600k lines of Rust, 2023) moved its CI off Cargo. The downsides they listed: verbose BUILD files, no `cargo check` equivalent, weak rust-analyzer support, and no `cargo publish`. They published no before/after timings ([mmapped.blog](https://mmapped.blog/posts/17-scaling-rust-builds-with-bazel)).
  - OpenAI Codex added Bazel next to Cargo in January 2026. "Cargo remains the source of truth", and Bazel uses BuildBuddy for remote caching and execution ([bazel.md](https://github.com/openai/codex/blob/main/codex-rs/docs/bazel.md)).
- **Buck2:** Meta's build system, written in Rust. crates.io dependencies come in through a tool called Reindeer ([Meta](https://engineering.fb.com/2023/04/06/open-source/buck2-open-source-large-scale-build-system/)).
- **Pants:** no first-class Rust support (according to maintainers in community chat).
- **moon:** supports Rust, rebuilds only what changed (using hashing), and offers a remote cache ([docs](https://moonrepo.dev/docs)).
- **Nx:** Rust only through a community plugin, `@monodon/rust`.

## 3. Keeping main green without pull requests

- **The rule.** Graydon Hoare's "Not Rocket Science Rule": "automatically maintain a repository that never fails its tests" (as paraphrased by Jane Street, 2014).
  - Jane Street also records the cost: checking changes one at a time takes at least m×n minutes for n changes of m minutes each, and their backlog reached 24 hours ([post](https://blog.janestreet.com/making-never-break-the-build-scale/)).
- **How rust-lang does it (bors)** ([rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/tests/ci.html)):
  - bors tests one merge commit at a time on `automation/bors/auto`. If it passes, main is fast-forwarded to it.
  - Manual merges are forbidden.
  - "Rollups" batch small changes so they are tested together.
- **Why it matters.** Two changes can each pass on their own and still break main together ([bors-ng README](https://github.com/bors-ng/bors-ng)). bors-ng is now deprecated in favor of GitHub's merge queue.
- **GitHub merge queue.** It only works with PRs. GitHub's docs say it is "available in any public repository owned by an organization, or in private repositories owned by organizations using GitHub Enterprise Cloud" ([docs source](https://github.com/github/docs/blob/main/data/reusables/gated-features/merge-queue.md)). It is not available to Hyleon.
- **What GitHub can enforce on the server.**
  - Branch protection and rulesets for private repos need Pro, Team or Enterprise ([protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches), [rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)).
  - With required status checks, GitHub's docs say that once checks pass, "commits must either be pushed to another branch and then merged or pushed directly to the protected branch". That is the staging-branch pattern below, enforced by the server, with no PRs needed.
  - GitHub Team costs $4 per user per month ([pricing](https://github.com/pricing)). I have not tested this flow.
- **Gerrit.** CI votes on a "Verified" label, and "Any -1 blocks submit" ([docs](https://gerrit-review.googlesource.com/Documentation/config-labels.html)).
- **The opposite approach: test after landing.** Firefox lands changes on an `autoland` branch, runs a subset of tests, and people called sheriffs back out bad changes and close the tree ([docs](https://firefox-source-docs.mozilla.org/testing/automated-testing/tree-closure.html)).
- **Staging-branch pattern on GitHub Actions.** You push to `staging`. CI runs the checks. A final job pushes the tested commit to `main` as a fast-forward.
  - Pushes made with `GITHUB_TOKEN` don't start new workflow runs ([docs](https://docs.github.com/en/actions/concepts/security/github_token)). So main is not tested a second time (no waste), but anything triggered by a push to main won't fire either.
  - With `concurrency` and `cancel-in-progress`, quick successive pushes are batched like rollups. The commits in between are not tested individually.
  - On the Free plan this is a habit, not a rule: nothing stops a direct push to main.

## 4. Tiered CI in real projects

| Project | How main is gated | Speed levers | Measured median wall-clock, successful runs |
|---|---|---|---|
| rust-lang/rust | bors queue with a full "auto" build | 5-job PR tier ("about 40 minutes" per the dev guide), rollups, sccache on S3 | auto 193 min (n=40); try 62.5 min (n=26) |
| Zed | merge queue | PRs test only affected crates via `rdeps()`; full suite on main; 16-vCPU Namespace runners; sccache on R2; 60-min job timeouts | PR 12.2 min, merge queue 7.3, main 12.8 |
| Ruff | PR checks | path-based `determine_changes` job; cache saved only from main; line-tables-only debuginfo; Depot 8–16-core runners; job timeouts of 5–20 min | PR 12.5, main 12.6 |
| uv | PR checks | central [`plan`](https://github.com/astral-sh/uv/blob/main/.github/workflows/plan.yml) job decides what runs; Windows tests split 3 ways; JUnit reports | PR 20.7, main 21.3 |
| Bevy | merge queue | caches built only by a [separate workflow](https://github.com/bevyengine/bevy/blob/main/.github/workflows/update-caches.yml) (on push to main and daily), CI only restores them; [platform validation](https://github.com/bevyengine/bevy/blob/main/.github/workflows/validation-jobs.yml) only in the merge queue | PR 30.2, merge queue 30.3 |
| Tauri | PR checks | separate [workflows per area](https://github.com/tauri-apps/tauri/blob/dev/.github/workflows/test-core.yml) with `paths:` filters; cache saved from `dev` by one matrix entry | `test core` on dev 28.3 (p10 11, p90 50) |
| Deno | PR checks | [generated workflow](https://github.com/denoland/deno/blob/main/.github/workflows/ci.generated.yml); benchmarks only on main or with a `ci-bench` label; caches saved only from main | PR 64.7, main 121.6 |

The tiers these projects use:
- **Per push or PR:** change detection decides what runs.
- **Gate:** expensive checks run only in the merge queue (Bevy) or the bors auto build (rust-lang).
- **Scheduled:** Zed builds nightly releases at 02:00 and 14:00 UTC ([workflow](https://github.com/zed-industries/zed/blob/main/.github/workflows/release_nightly.yml)). Ruff fuzzes daily ([workflow](https://github.com/astral-sh/ruff/blob/main/.github/workflows/daily_fuzz.yaml)).
- **Opt-in by label:** Deno's benchmarks.

Firefox is the extreme case (Mozilla Hacks, 2020-07-09, [post](https://hacks.mozilla.org/2020/07/testing-firefox-more-efficiently-with-machine-learning/)):
- Running every test everywhere would mean about 2.3 billion test files per day.
- Even with optimizations, CI used about 10 compute-years per day.
- Machine-learning test selection cut test tasks on autoland by 70% versus their previous approach, and by almost 99% versus running everything.
- Full runs still happen periodically, and sheriffs backfill missed tests.

## 5. Compile-time settings that matter in CI

- **Linker:** rust-lld has been the default for `x86_64-unknown-linux-gnu` since Rust 1.90.0 (2025-09-18). On a ripgrep debug build, linking was 7× faster, incremental rebuilds 40% faster and from-scratch builds 20% faster ([blog](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable)). wasm32 already used rust-lld.
- **Debug info:** the Cargo guide recommends `debug = "line-tables-only"` for your own crates and `false` for dependencies ([guide](https://doc.rust-lang.org/cargo/guide/build-performance.html)). Ruff uses line-tables-only. Bevy and Tauri turn it off completely; Tauri's comment says full debug info adds "unnecessary bloat to the target folder, decreasing cache efficiency".
- **`CARGO_INCREMENTAL=0`:** set by rust-cache, Zed, Ruff, uv and Bevy, and required by sccache. matklad's reasoning: CI builds are close to from-scratch builds, and incremental mode adds I/O and target size ([post](https://matklad.github.io/2021/09/04/fast-rust-builds.html)).
- **Warnings:** matklad recommends `-D warnings` in `RUSTFLAGS` rather than `#![deny]` in code. Bevy marks its `RUSTFLAGS` "Keep in sync between all workflows" — likely because artifacts built with different flags aren't reused.
- **Crate structure:** split crates for parallel builds, keep proc-macros and serde at the edges, and limit the number of test binaries, since linking cost grows with them (matklad).
- **LTO and codegen-units=1:** these belong in the release tier, not in test builds ([corrode](https://corrode.dev/blog/tips-for-faster-ci-builds/)).
- **Doctests:** edition 2024 combines doctests into a single binary instead of compiling each one separately ([edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/rustdoc-doctests.html)).
- **Measuring:** `cargo build --timings` has been stable since Rust 1.60 (2022-04-07). Cranelift, `-Zthreads` and native feature unification are nightly-only, so they don't fit pinned stable toolchains.
- **Runner size:** private-repo Linux runners have 2 vCPU, 8 GB RAM and a 14 GB SSD; public repos get 4 vCPU and 16 GB ([docs](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)). The fast projects pay for 8–16+ vCPU runners. Depot claims its runners are "30% faster and 50% cheaper" (vendor claim).

## 6. CI time budgets and observability

- **Target:** Martin Fowler (revised 2024-01-18): "the XP guideline of a ten minute build is perfectly within reason". He recommends a fast commit build plus a slower second stage ([article](https://martinfowler.com/articles/continuousIntegration.html)).
- **Hard limits:** GitHub's default job timeout is 360 minutes ([docs](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax)). Ruff sets 5–20 minutes per job, Zed 60, Deno up to 240. nextest's `slow-timeout` with `terminate-after` caps individual tests.
- **Built-in dashboard:** GitHub Actions Performance Metrics became generally available on 2025-03-14 for all Cloud plans. It shows run times, queue times and failure rates for up to a year ([changelog](https://github.blog/changelog/2025-03-14-actions-performance-metrics-are-generally-available-and-enterprise-level-metrics-are-in-public-preview)).
- **Tracking regressions:** rust-lang/rust posts a report after each merge listing the biggest job-duration changes, with a note that timings are noisy ([post-merge.yml](https://github.com/rust-lang/rust/blob/main/.github/workflows/post-merge.yml)). uv uploads nextest JUnit results as artifacts.
- **Do-it-yourself:** the same API queries I used here could run on a schedule and alert when the median passes a budget. Vendor dashboards (Datadog, Depot, Trunk) exist, but I did not evaluate them.

## What fits Hyleon now vs at scale

**Now (one job of about 40 s):**
1. **Keep the single job.** A standard private Linux runner costs $0.006 per minute, and each job is rounded up to a whole minute ([pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing)). Free includes $12 a month, about 2,000 such minutes. One 40-s job costs 1 billed minute; four parallel 40-s jobs cost 4.
2. **Write the time budget into the workflow.** Set `timeout-minutes` on the job (around 10, per Fowler) and a nextest `slow-timeout`. Check Performance Metrics now and then.
3. **Cache hygiene:**
   - Use rust-cache with `save-if` set to the branch that gates main.
   - Turn debug info off, or use line-tables-only, in CI.
   - Keep `RUSTFLAGS` identical across all CI jobs.
   - Use edition 2024 for combined doctests. I did not check which edition the crates use now.
4. **Add path filters when the website and data packs arrive.** Following the Tauri and Ruff pattern, an edit to the Astro site then wouldn't compile Rust.
5. **Choose how strictly main stays green** (your call):
   - a) **Current approach:** push to main, CI runs afterwards, fix forward. Costs nothing extra; main can be red for a while.
   - b) **Staging branch:** CI fast-forwards main after the checks pass. Same minutes; main only moves when green; kept by habit on Free.
   - c) **Option b plus GitHub Team** ($4 per user per month) with a ruleset requiring status checks on main. Enforced by the server, still no PRs.

**As the codebase grows.** The thresholds below are my own judgment, not from a source.
- **Push CI over about 5 minutes:** move slow work to a nightly tier that skips if main hasn't changed. Candidates: macOS and Windows desktop builds, release builds, long property or fuzz tests, and `cargo deny check advisories` (new advisories appear without any code change). On tags, add a release tier with LTO and signing.
- **Over about 10 minutes:**
  - Move sccache to R2, which shares compiled crates across jobs, survives lockfile bumps, and has free egress.
  - Test only affected crates with nextest `rdeps()` (Zed's script is about 60 lines of shell) or with the determinator, and keep full runs in the nightly tier.
- **Over about 20–30 minutes:** move to bigger runners (GitHub larger runners, or vendors such as Depot or Namespace, whose claims are vendor claims), and use nextest archive plus partitioning.
- **Bazel or Buck2:** only if the repo becomes multi-language and needs hermetic builds or remote execution that Cargo can't provide. Cargo-based projects much larger than Hyleon keep PR CI at about 10–30 minutes.
- **Self-hosting:** a runner on your Mac is excluded by your constraint. A cloud VM is possible: self-hosted minutes are still free, after GitHub postponed a proposed $0.002/min fee in December 2025 ([The Register](https://www.theregister.com/2025/12/17/github_charge_dev_own_hardware/), a secondary source).

## Limits

- **Durations are my own measurement.** I used GitHub's REST API with `run_started_at` to `updated_at` on successful runs, n=26–40 per row. This is wall-clock time, including job queueing and re-runs, not compute minutes.
  - Most runs fall in October 2026; Deno's main-branch runs span July–October.
  - rust-lang's PR workflow measured 87 min, against the guide's "about 40 minutes" for the PR jobs.
  - I can't explain why Zed's merge-queue runs are faster than its PR runs. Warm caches are a guess.
- **Graydon's original post** (graydon2.dreamwidth.org/1597.html) returns 404, and web.archive.org was blocked. The rule is quoted from Jane Street's paraphrase.
- **GitHub plan restrictions** (merge queue, rulesets, protected branches) come from GitHub's docs source files, because the rendered pages didn't show them. I did not verify which plans can use larger runners. The self-hosted fee status relies on secondary sources.
- **Unverified numbers:**
  - The sccache benchmarks are anecdotes from single machines, and I found no independent head-to-head of rust-cache against sccache.
  - hakari's speedup claims cite no benchmark.
  - I found no independent before/after CI timings for a Bazel migration.
  - The Firefox figures are from 2020.
  - I did not confirm when rust-lang switched from homu to the new bors.
- **Nothing was tested.** No workflow, ruleset or fast-forward job was tried. The staging pattern and option 5c are taken from the docs only.
