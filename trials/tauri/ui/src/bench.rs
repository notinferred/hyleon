//! T-native and T-wasm: the end-case screen in Leptos 0.8, the same page as variant D. Each part's
//! position is its own signal and the DOM updates straight from it (Leptos has no virtual DOM), so
//! a drag touches only the part and its wires. The core runs either in this wasm (T-wasm) or
//! natively behind Tauri commands (T-native), where every call crosses the bridge.

use bench_core::{Command, EDITED, Finding, Project, Row, Scene, Severity, Size};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::json;

use crate::{Settings, bridge};

const BENCH_CSS: &str = include_str!("../../../bench.css");
const PART_WIDTH: f32 = 80.0;
const PART_HEIGHT: f32 = 24.0;
const LIST_LIMIT: usize = 500;

#[derive(Clone, Copy, PartialEq)]
struct View {
    x: f32,
    y: f32,
    zoom: f32,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Model {
    /// The core runs natively, behind Tauri commands.
    native: bool,
    slow_ms: u64,
    /// The core, when it runs here.
    project: StoredValue<Option<Project>>,
    scene: RwSignal<Option<Scene>>,
    positions: RwSignal<Vec<RwSignal<(f32, f32)>>>,
    diagram_rev: RwSignal<u32>,
    findings: RwSignal<Vec<Finding>>,
    findings_rev: RwSignal<u32>,
    page: RwSignal<u32>,
    sheet: RwSignal<String>,
    overview: RwSignal<Option<Vec<String>>>,
    rows: RwSignal<(Vec<Row>, usize)>,
    rows_rev: RwSignal<u32>,
    view: RwSignal<View>,
    gesture: StoredValue<Option<Gesture>>,
    reloads: StoredValue<u64>,
}

fn expect<T>(result: Result<T, String>) -> T {
    result.unwrap_or_else(|e| panic!("the native core failed: {e}"))
}

impl Model {
    fn new(native: bool, slow_ms: u64) -> Model {
        Model {
            native,
            slow_ms,
            project: StoredValue::new(None),
            scene: RwSignal::new(None),
            positions: RwSignal::new(Vec::new()),
            diagram_rev: RwSignal::new(0),
            findings: RwSignal::new(Vec::new()),
            findings_rev: RwSignal::new(0),
            page: RwSignal::new(0),
            sheet: RwSignal::new(String::new()),
            overview: RwSignal::new(None),
            rows: RwSignal::new((Vec::new(), 0)),
            rows_rev: RwSignal::new(0),
            view: RwSignal::new(View {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            }),
            gesture: StoredValue::new(None),
            reloads: StoredValue::new(0),
        }
    }

    fn local<T>(self, f: impl FnOnce(&mut Project) -> T) -> T {
        self.project
            .try_update_value(|p| f(p.as_mut().expect("the project is open")))
            .expect("the model lives as long as the page")
    }

    async fn open(self, size: Size) {
        let (scene, findings) = if self.native {
            expect(bridge::invoke("core_open", &json!({})).await)
        } else {
            let project = Project::generate(size, 1);
            let opened = (project.scene(), project.check());
            self.project.set_value(Some(project));
            opened
        };
        self.show(scene, findings).await;
    }

    /// Shows a newly opened or reloaded project everywhere at once.
    async fn show(self, scene: Scene, findings: Vec<Finding>) {
        let sheet = self.sheet_svg(0).await;
        let rows = self.fetch_rows(String::new()).await;
        self.positions.set(
            scene
                .parts
                .iter()
                .map(|p| RwSignal::new((p.x, p.y)))
                .collect(),
        );
        self.scene.set(Some(scene));
        self.findings.set(findings);
        self.findings_rev.update(|r| *r += 1);
        self.sheet.set(sheet);
        self.rows.set(rows);
        self.rows_rev.update(|r| *r += 1);
        self.diagram_rev.update(|r| *r += 1);
    }

    fn move_part(self, part: u32, x: f32, y: f32) {
        let position = self.positions.with_untracked(|p| p[part as usize]);
        if self.native {
            spawn_local(async move {
                let () = expect(
                    bridge::invoke("core_move", &json!({ "part": part, "x": x, "y": y })).await,
                );
                position.set((x, y));
            });
        } else {
            self.local(|p| p.apply(Command::Move { part, x, y }));
            position.set((x, y));
        }
    }

    fn set_address(self, address: u8) {
        if self.native {
            spawn_local(async move {
                let findings = expect(
                    bridge::invoke("core_set_address", &json!({ "address": address })).await,
                );
                self.show_findings(findings);
            });
        } else {
            let findings = self.local(|p| {
                p.apply(Command::SetAddress {
                    part: EDITED,
                    address,
                });
                busy_wait(self.slow_ms);
                p.check()
            });
            self.show_findings(findings);
        }
    }

    fn show_findings(self, findings: Vec<Finding>) {
        self.findings.set(findings);
        self.findings_rev.update(|r| *r += 1);
    }

    async fn sheet_svg(self, page: u16) -> String {
        if self.native {
            expect(bridge::invoke("core_sheet", &json!({ "page": page })).await)
        } else {
            self.local(|p| p.sheet_svg(page))
        }
    }

    async fn fetch_rows(self, filter: String) -> (Vec<Row>, usize) {
        if self.native {
            expect(bridge::invoke("core_rows", &json!({ "filter": filter })).await)
        } else {
            self.local(|p| p.rows(&filter, LIST_LIMIT))
        }
    }

    fn next_page(self) {
        spawn_local(async move {
            let next = self.page.get_untracked() + 1;
            let pages =
                self.scene
                    .with_untracked(|s| s.as_ref().map_or(1, |s| s.pages)) as u32;
            let sheet = self.sheet_svg((next % pages) as u16).await;
            self.sheet.set(sheet);
            self.page.set(next);
        });
    }

    fn toggle_overview(self) {
        if self.overview.with_untracked(Option::is_some) {
            self.overview.set(None);
            return;
        }
        spawn_local(async move {
            let sheets = if self.native {
                expect(bridge::invoke("core_overview", &json!({})).await)
            } else {
                self.local(|p| (0..p.pages).map(|page| p.sheet_svg(page)).collect())
            };
            self.overview.set(Some(sheets));
        });
    }

    fn filter(self, text: String) {
        spawn_local(async move {
            let rows = self.fetch_rows(text).await;
            self.rows.set(rows);
            self.rows_rev.update(|r| *r += 1);
        });
    }

    /// An outside write: the other generated project, read back from bytes. For T-native the core
    /// reads them natively; for T-wasm they cross the bridge and are read here.
    fn reload(self) {
        let reloads = self.reloads.get_value();
        self.reloads.set_value(reloads + 1);
        let seed = if reloads % 2 == 0 { 2 } else { 1 };
        spawn_local(async move {
            let (scene, findings) = if self.native {
                expect(bridge::invoke("core_reload", &json!({ "seed": seed })).await)
            } else {
                let bytes = if bridge::in_tauri() {
                    expect(bridge::invoke_bytes("project_bytes", &json!({ "seed": seed })).await)
                } else {
                    let size = self.local(|p| p.size);
                    Project::generate(size, seed).to_bytes()
                };
                let project = expect(Project::from_bytes(&bytes));
                let reloaded = (project.scene(), project.check());
                self.project.set_value(Some(project));
                reloaded
            };
            self.show(scene, findings).await;
        });
    }
}

/// `--slow`: a busy wait before each check, so the harness can prove it measures what it should.
fn busy_wait(ms: u64) {
    if ms == 0 {
        return;
    }
    let Some(clock) = web_sys::window().and_then(|w| w.performance()) else {
        return;
    };
    let until = clock.now() + ms as f64;
    while clock.now() < until {}
}

fn severity_class(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

#[component]
pub fn BenchScreen(settings: Settings) -> impl IntoView {
    let model = Model::new(settings.core == "native", settings.slow_ms);
    provide_context(model);
    let size = Size::parse(&settings.size).unwrap_or(Size::Panel);
    let variant = if settings.core == "native" {
        "T-native"
    } else {
        "T-wasm"
    };
    spawn_local(async move {
        model.open(size).await;
        let config = json!({
            "variant": variant,
            "size": size.name(),
            "pages": size.pages(),
            "slowMs": settings.slow_ms,
            "startEpochMs": settings.start_epoch_ms,
        });
        if settings.mode != "bench" {
            return;
        }
        let results = bridge::bench(&config).await;
        if bridge::in_tauri() {
            let _: Result<(), String> =
                bridge::invoke("bench_done", &json!({ "results": results.to_string() })).await;
        } else if let Some(window) = web_sys::window() {
            let _ = post_results(&window, &results.to_string()).await;
        }
    });

    view! {
        <style>{BENCH_CSS}</style>
        <div class="bench">
            <Controls />
            <Diagram />
            <div class="side">
                <Findings />
                <List />
            </div>
            <Pages />
        </div>
    }
}

/// The web build posts its results to the server that serves it (`trials/bench-web.py`).
async fn post_results(window: &web_sys::Window, body: &str) -> Result<(), wasm_bindgen::JsValue> {
    let fetch = js_sys::Reflect::get(window, &"fetch".into())?;
    let init = js_sys::Object::new();
    js_sys::Reflect::set(&init, &"method".into(), &"POST".into())?;
    js_sys::Reflect::set(&init, &"body".into(), &body.into())?;
    let promise = js_sys::Function::from(fetch).call2(window, &"/results".into(), &init)?;
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise)).await?;
    Ok(())
}

#[component]
fn Controls() -> impl IntoView {
    let m = expect_context::<Model>();
    view! {
        <div class="controls">
            <label>
                "Address of P-0002"
                <input
                    id="edit-address"
                    type="number"
                    on:input=move |ev| {
                        if let Ok(address) = event_target_value(&ev).parse::<u8>() {
                            m.set_address(address);
                        }
                    }
                />
            </label>
            <button id="next-page" on:click=move |_| m.next_page()>"Next page"</button>
            <button id="overview-toggle" on:click=move |_| m.toggle_overview()>"All pages"</button>
            <label>
                "Filter"
                <input id="filter" on:input=move |ev| m.filter(event_target_value(&ev)) />
            </label>
            <button id="reload" on:click=move |_| m.reload()>"Reload"</button>
        </div>
    }
}

#[component]
fn Diagram() -> impl IntoView {
    let m = expect_context::<Model>();
    let size = move || {
        m.scene
            .with(|s| s.as_ref().map_or((0.0, 0.0), |s| (s.width, s.height)))
    };
    let content = move || {
        m.diagram_rev.track();
        let positions = m.positions.get_untracked();
        m.scene.with_untracked(|scene| {
            scene.as_ref().map(|scene| {
                let wires = scene
                    .wires
                    .iter()
                    .map(|w| {
                        let (a, b) = (positions[w.from as usize], positions[w.to as usize]);
                        view! {
                            <line
                                class="wire"
                                x1=move || a.get().0 + PART_WIDTH / 2.0
                                y1=move || a.get().1 + PART_HEIGHT / 2.0
                                x2=move || b.get().0 + PART_WIDTH / 2.0
                                y2=move || b.get().1 + PART_HEIGHT / 2.0
                            />
                        }
                    })
                    .collect_view();
                let parts = scene
                    .parts
                    .iter()
                    .map(|p| {
                        let (id, at, tag) = (p.id, positions[p.id as usize], p.tag.clone());
                        view! {
                            <g
                                id=format!("part-{id}")
                                class="part"
                                data-x=move || at.get().0.round() as i64
                                transform=move || {
                                    let (x, y) = at.get();
                                    format!("translate({x} {y})")
                                }
                                on:pointerdown=move |ev: web_sys::PointerEvent| {
                                    ev.stop_propagation();
                                    let from = (ev.client_x() as f64, ev.client_y() as f64);
                                    m.gesture.set_value(Some(Gesture::Drag { part: id, from, start: at.get_untracked() }));
                                }
                            >
                                <rect width=PART_WIDTH height=PART_HEIGHT rx="3" />
                                <text x="6" y="16">{tag}</text>
                            </g>
                        }
                    })
                    .collect_view();
                view! {
                    <g>{wires}</g>
                    <g>{parts}</g>
                }
            })
        })
    };

    view! {
        <div
            id="diagram-frame"
            on:pointerdown=move |ev: web_sys::PointerEvent| {
                let view = m.view.get_untracked();
                let from = (ev.client_x() as f64, ev.client_y() as f64);
                m.gesture.set_value(Some(Gesture::Pan { from, start: (view.x, view.y) }));
            }
            on:pointermove=move |ev: web_sys::PointerEvent| {
                let at = (ev.client_x() as f64, ev.client_y() as f64);
                match m.gesture.get_value() {
                    Some(Gesture::Drag { part, from, start }) => {
                        let zoom = m.view.get_untracked().zoom;
                        let x = start.0 + (at.0 - from.0) as f32 / zoom;
                        let y = start.1 + (at.1 - from.1) as f32 / zoom;
                        m.move_part(part, x, y);
                    }
                    Some(Gesture::Pan { from, start }) => {
                        m.view.update(|v| {
                            v.x = start.0 + (at.0 - from.0) as f32;
                            v.y = start.1 + (at.1 - from.1) as f32;
                        });
                    }
                    None => {}
                }
            }
            on:pointerup=move |_| m.gesture.set_value(None)
            on:wheel=move |ev: web_sys::WheelEvent| {
                ev.prevent_default();
                let factor = if ev.delta_y() < 0.0 { 1.1 } else { 1.0 / 1.1 };
                m.view.update(|v| v.zoom *= factor);
            }
        >
            <svg
                id="diagram"
                data-ready=move || if m.scene.with(Option::is_some) { "1" } else { "0" }
                data-rev=move || m.diagram_rev.get()
                width=move || size().0
                height=move || size().1
            >
                <g
                    id="diagram-view"
                    data-x=move || m.view.get().x.round() as i64
                    transform=move || {
                        let v = m.view.get();
                        format!("translate({} {}) scale({})", v.x, v.y, v.zoom)
                    }
                >
                    {content}
                </g>
            </svg>
        </div>
    }
}

#[component]
fn Findings() -> impl IntoView {
    let m = expect_context::<Model>();
    let count = move || m.findings.with(Vec::len);
    view! {
        <div id="findings" data-rev=move || m.findings_rev.get() data-count=count>
            <h2>"Findings (" {count} ")"</h2>
            <ul>
                <For
                    each=move || m.findings.get()
                    key=|f| format!("{}:{}", f.id, f.what)
                    children=|f| view! { <li class=severity_class(f.severity)>{f.what} " " {f.place}</li> }
                />
            </ul>
        </div>
    }
}

#[component]
fn List() -> impl IntoView {
    let m = expect_context::<Model>();
    view! {
        <div id="rows" data-rev=move || m.rows_rev.get()>
            <h2>{move || m.rows.with(|r| r.1)} " parts and connections"</h2>
            <table>
                <tbody>
                    <For
                        each=move || m.rows.get().0
                        key=|r| r.key.clone()
                        children=|r| {
                            view! {
                                <tr>
                                    <td>{r.tag}</td>
                                    <td>{r.kind}</td>
                                    <td>{r.page}</td>
                                    <td>{r.detail}</td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Pages() -> impl IntoView {
    let m = expect_context::<Model>();
    view! {
        <div class="pages">
            {move || match m.overview.get() {
                Some(sheets) => {
                    let count = sheets.len();
                    view! {
                        <div id="overview" data-count=count>
                            {sheets
                                .into_iter()
                                .map(|svg| view! { <div class="thumb" inner_html=svg></div> })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! { <div id="sheet" data-page=move || m.page.get() inner_html=move || m.sheet.get()></div> }
                        .into_any()
                }
            }}
        </div>
    }
}
