use web_sys::js_sys;
use web_sys::wasm_bindgen::JsValue;

pub fn expose_to_window(v: &JsValue, name: &str) -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    js_sys::Reflect::set(&window, &JsValue::from_str(name), v)
}
