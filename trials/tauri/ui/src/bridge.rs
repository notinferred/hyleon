//! Tauri's JavaScript bridge, bound from Rust. This is the layer between the UI and a native core:
//! every call is serialized and asynchronous. On the web there's no Tauri, and `in_tauri` says so.

use js_sys::{Function, Reflect, Uint8Array};
use leptos::task::spawn_local;
use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn invoke_js(command: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
    async fn listen_js(event: &str, handler: &Function) -> JsValue;

    /// Defined by `trials/bench.js`.
    #[wasm_bindgen(js_name = hyleonBench)]
    fn hyleon_bench(config: JsValue) -> js_sys::Promise;
}

pub fn in_tauri() -> bool {
    web_sys::window().is_some_and(|w| Reflect::has(&w, &"__TAURI__".into()).unwrap_or(false))
}

fn to_js<T: Serialize>(value: &T) -> Result<JsValue, String> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| e.to_string())
}

pub async fn invoke<A: Serialize, R: DeserializeOwned>(
    command: &str,
    args: &A,
) -> Result<R, String> {
    let value = invoke_js(command, to_js(args)?)
        .await
        .map_err(|e| format!("{command}: {e:?}"))?;
    serde_wasm_bindgen::from_value(value).map_err(|e| format!("{command}: {e}"))
}

/// Sends a line to the app's `--log` file. Desktop webviews have no console to read in CI.
pub fn log(message: String) {
    if in_tauri() {
        spawn_local(async move {
            let _: Result<(), String> =
                invoke("ui_log", &serde_json::json!({ "message": message })).await;
        });
    }
}

/// For commands that answer with raw bytes rather than JSON.
pub async fn invoke_bytes<A: Serialize>(command: &str, args: &A) -> Result<Vec<u8>, String> {
    let value = invoke_js(command, to_js(args)?)
        .await
        .map_err(|e| format!("{command}: {e:?}"))?;
    Ok(Uint8Array::new(&value).to_vec())
}

/// Calls `handler` with each string payload of `event`, for as long as the page lives.
pub fn listen(event: &'static str, mut handler: impl FnMut(String) + 'static) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
        if let Some(payload) = Reflect::get(&e, &"payload".into())
            .ok()
            .and_then(|p| p.as_string())
        {
            handler(payload);
        }
    });
    let function: Function = closure.as_ref().unchecked_ref::<Function>().clone();
    closure.forget();
    spawn_local(async move {
        listen_js(event, &function).await;
    });
}

/// Runs the timing script and returns its results.
pub async fn bench(config: &serde_json::Value) -> serde_json::Value {
    let Ok(config) = to_js(config) else {
        return serde_json::json!({ "errors": ["config"] });
    };
    match JsFuture::from(hyleon_bench(config)).await {
        Ok(results) => serde_wasm_bindgen::from_value(results)
            .unwrap_or_else(|e| serde_json::json!({ "errors": [e.to_string()] })),
        Err(e) => serde_json::json!({ "errors": [format!("{e:?}")] }),
    }
}
