mod bpk1block;
mod bpk1file;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

fn create_frontend_error(service_name: &str, error: &str) -> JsError {
    JsError::new(format!("{} failed: {}", service_name, error).as_str())
}
