use js_sys::global;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from InPage Script");

    let global: Object = global();

    let add_closure = Closure::wrap(Box::new(add_one) as Box<dyn Fn(u8) -> u8>);
    let add_closure = add_closure.into_js_value();

    js_sys::Reflect::set(&global, &JsValue::from("add_one"), &add_closure).unwrap();
}

#[wasm_bindgen]
pub fn add_one(number: u8) -> u8 {
    number + 1
}
