use serde_json::json;
use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

use crate::scenarios::Scenario;
use crate::script;

/// Runs a scenario script and returns `{ ok, report, log, payments }` or `{ ok: false, error }` as JSON.
/// `progress` is called with the fraction of the simulated day done.
#[wasm_bindgen]
pub fn run(source: &str, progress: &js_sys::Function) -> String {
    let report = |f: f64| drop(progress.call1(&JsValue::NULL, &JsValue::from_f64(f)));
    match script::run_with_progress(source, None, report) {
        Ok(r) => json!({ "ok": true, "report": r.report, "log": r.log, "payments": r.payments }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
    .to_string()
}

/// Parses without running: `{ ok, payments }` or `{ ok: false, error }`.
#[wasm_bindgen]
pub fn check(source: &str) -> String {
    match script::check(source) {
        Ok(n) => json!({ "ok": true, "payments": n }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
    .to_string()
}

#[wasm_bindgen]
pub fn templates() -> String {
    let list: Vec<_> = Scenario::ALL
        .into_iter()
        .map(|s| json!({ "id": s.name(), "title": s.title(), "source": s.source() }))
        .collect();
    json!(list).to_string()
}
