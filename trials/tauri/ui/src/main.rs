//! The Leptos UI for the T-02 comparison: variants T-native and T-wasm, and the trial screen. In
//! Tauri it asks the app how it was started; on the web it reads its address
//! (`?size=building&bench=1&core=wasm`).

mod bench;
mod bridge;
mod questions;
mod trial;

use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;

/// How the app was started.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// `bench` plays the timing script, `screen` shows the bench screen to try by hand, `trial`
    /// shows the trial screen.
    pub mode: String,
    pub size: String,
    /// `native` or `wasm`: where the core runs.
    pub core: String,
    /// Time only opening, dragging and editing (the capacity test).
    #[serde(default)]
    pub capacity: bool,
    pub slow_ms: u64,
    pub start_epoch_ms: Option<f64>,
}

impl Settings {
    fn from_query(query: &str) -> Settings {
        let value = |key: &str| {
            query
                .trim_start_matches('?')
                .split('&')
                .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
        };
        let mode = match (value("bench"), value("size")) {
            (Some("1"), _) => "bench",
            (_, Some(_)) => "screen",
            _ => "trial",
        };
        Settings {
            mode: mode.to_string(),
            size: value("size").unwrap_or("panel").to_string(),
            core: "wasm".to_string(),
            capacity: value("capacity") == Some("1"),
            slow_ms: value("slow").and_then(|s| s.parse().ok()).unwrap_or(0),
            start_epoch_ms: None,
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| bridge::log(format!("panic: {info}"))));
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let settings = RwSignal::new(None::<Settings>);
    spawn_local(async move {
        let found = if bridge::in_tauri() {
            bridge::invoke("settings", &serde_json::json!({}))
                .await
                .ok()
        } else {
            web_sys::window()
                .and_then(|w| w.location().search().ok())
                .map(|query| Settings::from_query(&query))
        };
        settings.set(found);
    });
    move || match settings.get() {
        Some(settings) if settings.mode == "trial" => view! { <trial::TrialScreen /> }.into_any(),
        Some(settings) => view! { <bench::BenchScreen settings /> }.into_any(),
        None => view! { <p>"Loading…"</p> }.into_any(),
    }
}
