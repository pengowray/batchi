use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// Get the Tauri internals object, if running in Tauri.
pub fn get_tauri_internals() -> Option<JsValue> {
    let window = web_sys::window()?;
    let tauri = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI_INTERNALS__")).ok()?;
    if tauri.is_undefined() {
        None
    } else {
        Some(tauri)
    }
}

/// Invoke a Tauri command and return the result as a JsValue.
pub async fn tauri_invoke(cmd: &str, args: &JsValue) -> Result<JsValue, String> {
    let tauri = get_tauri_internals().ok_or("Not running in Tauri")?;
    let invoke = js_sys::Reflect::get(&tauri, &JsValue::from_str("invoke"))
        .map_err(|_| "No invoke function")?;
    let invoke_fn = js_sys::Function::from(invoke);

    let promise_val = invoke_fn
        .call2(&tauri, &JsValue::from_str(cmd), args)
        .map_err(|e| format!("Invoke call failed: {:?}", e))?;

    let promise: js_sys::Promise = promise_val
        .dyn_into()
        .map_err(|_| "Result is not a Promise")?;

    JsFuture::from(promise)
        .await
        .map_err(|e| format!("Command '{}' failed: {:?}", cmd, e))
}

/// Invoke a Tauri command with no arguments.
pub async fn tauri_invoke_no_args(cmd: &str) -> Result<JsValue, String> {
    tauri_invoke(cmd, &js_sys::Object::new().into()).await
}
