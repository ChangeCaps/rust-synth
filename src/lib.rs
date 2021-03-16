mod app;

pub use app::*;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    let app = App::new().map_err(|e| format!("{}", e))?;
    eframe::start_web(canvas_id, Box::new(app))
}
