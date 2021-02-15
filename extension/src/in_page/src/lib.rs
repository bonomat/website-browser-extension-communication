use js_sys::global;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from InPage Script");

    let global: Object = global();

    let add_closure = Closure::wrap(Box::new(call_backend) as Box<dyn Fn(String)>);
    let add_closure = add_closure.into_js_value();

    js_sys::Reflect::set(&global, &JsValue::from("call_backend"), &add_closure).unwrap();
}

#[wasm_bindgen]
pub fn call_backend(txt: String) {
    let js_value = JsValue::from(txt);
    window.post_message(js_value);
    // log::info!("Received response: {:?}", response);
    // response.as_string().unwrap()
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Window, value: JsValue) -> JsValue;
}
