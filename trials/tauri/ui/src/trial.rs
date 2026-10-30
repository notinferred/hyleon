//! The T-02 trial screen in Leptos, to compare with the Dioxus one: the same question form, findings
//! and 5,752-element sheet, with the same accessibility and keys. In Tauri the native menu drives the
//! zoom, Check for Updates reports here, and `--self-test` hears about the first render.

use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::json;

use crate::bridge;
use crate::questions::{self, Field, Form, Kind};

const APP_CSS: &str = include_str!("../../../assets/trial/app.css");
const TRIAL_SHEET: &str = include_str!("../../../assets/trial/sheet.svg");
const TRIAL_FINDINGS: &str = include_str!("../../../assets/trial/findings.json");
const STEP: f64 = 1.25;
const NUDGE: f64 = 40.0;

/// How the sheet is viewed, shared so the menu and shortcuts can zoom it.
#[derive(Clone, Copy)]
struct SheetView {
    zoom: RwSignal<f64>,
    offset: RwSignal<(f64, f64)>,
    size: RwSignal<(f64, f64)>,
}

impl SheetView {
    fn zoom_at(self, factor: f64, at: (f64, f64)) {
        let old = self.zoom.get_untracked();
        let new = (old * factor).clamp(0.1, 20.0);
        let kept = new / old;
        let (x, y) = self.offset.get_untracked();
        self.zoom.set(new);
        self.offset
            .set((at.0 - (at.0 - x) * kept, at.1 - (at.1 - y) * kept));
    }

    fn centre(self) -> (f64, f64) {
        let (w, h) = self.size.get_untracked();
        (w / 2.0, h / 2.0)
    }

    fn zoom_in(self) {
        self.zoom_at(STEP, self.centre());
    }

    fn zoom_out(self) {
        self.zoom_at(1.0 / STEP, self.centre());
    }

    fn reset(self) {
        self.zoom.set(1.0);
        self.offset.set((0.0, 0.0));
    }

    fn pan_by(self, dx: f64, dy: f64) {
        self.offset.update(|(x, y)| {
            *x += dx;
            *y += dy;
        });
    }
}

#[component]
pub fn TrialScreen() -> impl IntoView {
    let view = SheetView {
        zoom: RwSignal::new(1.0),
        offset: RwSignal::new((0.0, 0.0)),
        size: RwSignal::new((0.0, 0.0)),
    };
    provide_context(view);
    let status = RwSignal::new(String::new());
    if bridge::in_tauri() {
        bridge::listen("menu", move |id| match id.as_str() {
            "zoom-in" => view.zoom_in(),
            "zoom-out" => view.zoom_out(),
            "zoom-to-fit" => view.reset(),
            _ => {}
        });
        bridge::listen("status", move |text| status.set(text));
        // After the first render: the self-test, if one was asked for.
        Effect::new(move |_| {
            spawn_local(async move {
                let _: Result<(), String> = bridge::invoke("rendered", &json!({})).await;
            });
        });
    }
    // On the web, Cmd on macOS and Ctrl elsewhere zoom the sheet (DESIGN.md, Familiarity); in Tauri
    // the native menu owns these keys.
    let shortcut = move |ev: web_sys::KeyboardEvent| {
        if bridge::in_tauri() || !(ev.meta_key() || ev.ctrl_key()) {
            return;
        }
        match ev.key().as_str() {
            "=" | "+" => view.zoom_in(),
            "-" => view.zoom_out(),
            "0" => view.reset(),
            _ => return,
        }
        ev.prevent_default();
    };

    view! {
        <style>{APP_CSS}</style>
        <div class="app" on:keydown=shortcut>
            <header class="bar">
                <h1>"Hyleon"</h1>
                <p class="build">"Trial build (Leptos + Tauri)"</p>
                <p class="status" role="status">{move || status.get()}</p>
            </header>
            <aside class="pane" aria-label="Questions">
                <Questions />
            </aside>
            <main class="sheet-pane">
                <Sheet />
            </main>
            <aside class="pane" aria-label="Findings">
                <Findings />
            </aside>
        </div>
    }
}

#[component]
fn Questions() -> impl IntoView {
    match questions::parse(questions::TRIAL_SCHEMA) {
        Ok(form) => view! { <QuestionForm form /> }.into_any(),
        Err(why) => view! { <p role="alert">{why}</p> }.into_any(),
    }
}

#[component]
fn QuestionForm(form: Form) -> impl IntoView {
    let count = form.fields.len();
    let answers: Vec<RwSignal<String>> = (0..count).map(|_| RwSignal::new(String::new())).collect();
    let problems: Vec<RwSignal<Option<String>>> = (0..count).map(|_| RwSignal::new(None)).collect();
    let summary = RwSignal::new(String::new());
    let fields = form.fields.clone();
    let (checked_answers, checked_problems) = (answers.clone(), problems.clone());
    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let mut open = 0;
        for (i, field) in fields.iter().enumerate() {
            let found = questions::problem(field, &checked_answers[i].get_untracked());
            open += usize::from(found.is_some());
            checked_problems[i].set(found);
        }
        summary.set(match open {
            0 => format!("{count} answers recorded."),
            1 => "1 answer needs attention.".to_string(),
            n => format!("{n} answers need attention."),
        });
    };

    view! {
        <form class="questions" aria-labelledby="questions-title" novalidate=true on:submit=submit>
            <h2 id="questions-title">{form.title.clone()}</h2>
            {form
                .fields
                .into_iter()
                .enumerate()
                .map(|(i, field)| view! { <FieldControl field answer=answers[i] problem=problems[i] /> })
                .collect_view()}
            <button type="submit">"Record answers"</button>
            <p role="status">{move || summary.get()}</p>
        </form>
    }
}

#[component]
fn FieldControl(
    field: Field,
    answer: RwSignal<String>,
    problem: RwSignal<Option<String>>,
) -> impl IntoView {
    let id = format!("q-{}", field.name);
    let help = format!("{id}-help");
    let error = format!("{id}-error");
    let has_help = field.description.is_some();
    let described = {
        let (help, error) = (help.clone(), error.clone());
        move || {
            let mut ids = Vec::new();
            if has_help {
                ids.push(help.clone());
            }
            if problem.with(Option::is_some) {
                ids.push(error.clone());
            }
            ids.join(" ")
        }
    };
    let invalid = move || {
        if problem.with(Option::is_some) {
            "true"
        } else {
            "false"
        }
    };
    let label = if field.required {
        format!("{} (required)", field.title)
    } else {
        field.title.clone()
    };
    let control = match field.kind.clone() {
        Kind::Choice(choices) => view! {
            <select
                id=id.clone()
                required=field.required
                aria-describedby=described
                aria-invalid=invalid
                on:change=move |ev| answer.set(event_target_value(&ev))
            >
                <option value="">"Choose…"</option>
                {choices
                    .into_iter()
                    .map(|c| {
                        let label = c.clone();
                        view! { <option value=c>{label}</option> }
                    })
                    .collect_view()}
            </select>
        }
        .into_any(),
        Kind::Bool => view! {
            <input
                id=id.clone()
                type="checkbox"
                aria-describedby=described
                on:change=move |ev| answer.set(event_target_checked(&ev).to_string())
            />
        }
        .into_any(),
        Kind::Number { whole, min, max } => view! {
            <input
                id=id.clone()
                type="number"
                inputmode=if whole { "numeric" } else { "decimal" }
                step=if whole { "1" } else { "any" }
                min=min.map(|m| m.to_string())
                max=max.map(|m| m.to_string())
                required=field.required
                aria-describedby=described
                aria-invalid=invalid
                on:input=move |ev| answer.set(event_target_value(&ev))
            />
        }
        .into_any(),
        Kind::Text => view! {
            <input
                id=id.clone()
                type="text"
                required=field.required
                aria-describedby=described
                aria-invalid=invalid
                on:input=move |ev| answer.set(event_target_value(&ev))
            />
        }
        .into_any(),
    };
    let class = if matches!(field.kind, Kind::Bool) {
        "field check"
    } else {
        "field"
    };

    view! {
        <div class=class>
            <label for=id>{label}</label>
            {control}
            {field.description.map(|d| view! { <p id=help class="help">{d}</p> })}
            {move || problem.get().map(|p| view! { <p id=error.clone() class="problem">{p}</p> })}
        </div>
    }
}

#[derive(Clone, Deserialize)]
struct Finding {
    severity: String,
    what: String,
    #[serde(rename = "where")]
    place: String,
    why: String,
    fix: String,
}

#[component]
fn Findings() -> impl IntoView {
    let findings: Vec<Finding> = match serde_json::from_str(TRIAL_FINDINGS) {
        Ok(findings) => findings,
        Err(e) => {
            return view! { <p role="alert">{format!("The findings couldn't be read: {e}")}</p> }
                .into_any();
        }
    };
    let count = findings.len();
    view! {
        <section class="findings" aria-labelledby="findings-title">
            <h2 id="findings-title">{format!("Findings ({count})")}</h2>
            // Without bullets, WebKit stops calling this a list, so the role says it again.
            <ul role="list">
                {findings
                    .into_iter()
                    .map(|f| {
                        let (word, shape) = match f.severity.as_str() {
                            "error" => ("Error", "■"),
                            "warning" => ("Warning", "▲"),
                            _ => ("Information", "●"),
                        };
                        view! {
                            <li class=format!("finding {}", word.to_lowercase())>
                                <p class="what">
                                    <span class="shape" aria-hidden="true">{shape}</span>
                                    <strong>{format!("{word}: ")}</strong>
                                    {f.what}
                                </p>
                                <p>{format!("Where: {}", f.place)}</p>
                                <p>{format!("Why: {}", f.why)}</p>
                                <p>{format!("Fix: {}", f.fix)}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
    .into_any()
}

/// Panning moves the drawn layer; zooming resizes the drawing so it's redrawn sharp, as in the
/// fixed Dioxus trial.
#[component]
fn Sheet() -> impl IntoView {
    let view = expect_context::<SheetView>();
    let grab = StoredValue::new(None::<(f64, f64)>);
    let frame = NodeRef::<leptos::html::Div>::new();
    let measure = move || {
        if let Some(el) = frame.get_untracked() {
            view.size
                .set((el.client_width() as f64, el.client_height() as f64));
        }
    };
    Effect::new(move |_| measure());

    view! {
        <div
            node_ref=frame
            class="sheet"
            tabindex="0"
            role="group"
            aria-label="Sheet E-101, drive bus and control wiring"
            aria-describedby="sheet-keys"
            on:focus=move |_| measure()
            on:keydown=move |ev: web_sys::KeyboardEvent| {
                match ev.key().as_str() {
                    "ArrowLeft" => view.pan_by(NUDGE, 0.0),
                    "ArrowRight" => view.pan_by(-NUDGE, 0.0),
                    "ArrowUp" => view.pan_by(0.0, NUDGE),
                    "ArrowDown" => view.pan_by(0.0, -NUDGE),
                    "+" | "=" if !(ev.meta_key() || ev.ctrl_key()) => view.zoom_in(),
                    "-" if !(ev.meta_key() || ev.ctrl_key()) => view.zoom_out(),
                    "0" if !(ev.meta_key() || ev.ctrl_key()) => view.reset(),
                    _ => return,
                }
                ev.prevent_default();
            }
            on:wheel=move |ev: web_sys::WheelEvent| {
                ev.prevent_default();
                let factor = if ev.delta_y() < 0.0 { STEP } else { 1.0 / STEP };
                view.zoom_at(factor, (ev.offset_x() as f64, ev.offset_y() as f64));
            }
            on:mousedown=move |ev: web_sys::MouseEvent| grab.set_value(Some((ev.client_x() as f64, ev.client_y() as f64)))
            on:mousemove=move |ev: web_sys::MouseEvent| {
                let Some((fx, fy)) = grab.get_value() else { return };
                let at = (ev.client_x() as f64, ev.client_y() as f64);
                view.pan_by(at.0 - fx, at.1 - fy);
                grab.set_value(Some(at));
            }
            on:mouseup=move |_| grab.set_value(None)
            on:mouseleave=move |_| grab.set_value(None)
        >
            <div
                class="canvas"
                style=move || {
                    let (x, y) = view.offset.get();
                    format!("transform: translate({x}px, {y}px); width: {}%", view.zoom.get() * 100.0)
                }
            >
                <div class="art" inner_html=TRIAL_SHEET></div>
            </div>
        </div>
        <p id="sheet-keys" class="hint">
            {move || format!("Arrow keys pan, plus and minus zoom, and 0 fits the sheet. Zoom {}%.", (view.zoom.get() * 100.0).round())}
        </p>
    }
}
