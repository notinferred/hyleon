//! The Dioxus app, for web and desktop. For now, the T-02 trial screen: questions, a sheet and
//! its findings.

#[cfg(feature = "desktop")]
pub mod desktop;
pub mod findings;
pub mod questions;
pub mod sheet;

use dioxus::prelude::*;

const APP_CSS: &str = include_str!("../assets/trial/app.css");

#[component]
pub fn App() -> Element {
    let view = use_context_provider(sheet::View::new);
    #[cfg(feature = "desktop")]
    let status = desktop::use_desktop(view);
    #[cfg(not(feature = "desktop"))]
    let status: Option<String> = None;
    let build = build_label();

    rsx! {
        style { {APP_CSS} }
        div { class: "app", onkeydown: move |evt| shortcut(view, evt),
            header { class: "bar",
                h1 { "Hyleon" }
                p { class: "build", "{build}" }
                p { class: "status", role: "status", {status} }
            }
            aside { class: "pane", aria_label: "Questions", questions::Questions {} }
            main { class: "sheet-pane", sheet::Sheet {} }
            aside { class: "pane", aria_label: "Findings", findings::Findings {} }
        }
    }
}

/// Cmd on macOS and Ctrl elsewhere zoom the sheet (DESIGN.md, Familiarity). On desktop the native
/// menu owns these keys, so only the web build handles them here. The page can't tell which
/// platform it runs on without a script, so either key is accepted: on each platform the other one
/// belongs to the operating system and never reaches the page.
fn shortcut(view: sheet::View, evt: KeyboardEvent) {
    let modifiers = evt.modifiers();
    if cfg!(feature = "desktop") || !(modifiers.meta() || modifiers.ctrl()) {
        return;
    }
    match evt.key() {
        Key::Character(c) if c == "=" || c == "+" => view.zoom_in(),
        Key::Character(c) if c == "-" => view.zoom_out(),
        Key::Character(c) if c == "0" => view.reset(),
        _ => return,
    }
    evt.prevent_default();
}

#[cfg(feature = "desktop")]
fn build_label() -> String {
    format!("Trial build {}", desktop::version())
}

#[cfg(not(feature = "desktop"))]
fn build_label() -> String {
    "Trial build (web)".to_string()
}
