use js_sys::{Object, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method)]
    pub fn open(this: &Window, url: String) -> Window;
}

#[wasm_bindgen]
extern "C" {
    pub type Browser;
    pub static browser: Browser;

    #[wasm_bindgen(method, getter)]
    pub fn windows(this: &Browser) -> Windows;

    #[wasm_bindgen(method, getter)]
    pub fn runtime(this: &Browser) -> Runtime;

}

#[wasm_bindgen]
extern "C" {
    pub type Sidebar;

    #[wasm_bindgen(method, getter)]
    pub fn open(this: &Sidebar) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type Windows;

    #[wasm_bindgen(method)]
    pub fn create(this: &Windows, info: &Object) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type Runtime;

    #[wasm_bindgen(method, getter, js_name = onMessage)]
    pub fn on_message(this: &Runtime) -> Event;
}

#[wasm_bindgen]
extern "C" {
    pub type Event;

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &Event, closure: &Closure<dyn Fn(JsValue) -> Promise>);

}

#[wasm_bindgen]
pub fn is_locked() -> bool {
    true
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("BS: Hello World");

    /*        let popup = Popup {
        url: "popup.html".to_string(),
        type_: "popup".to_string(),
        height: 200,
        width: 200,
    };
    let js_value = JsValue::from_serde(&popup).unwrap();
    let object = Object::try_from(&js_value).unwrap();
    let _x = browser.windows().create(&object);*/

    let func = |msg: JsValue| {
        log::info!("BS: Received from CS: {}", msg.as_string().unwrap());
        let response = JsValue::from("World");
        Promise::resolve(&response)
    };
    let closure = Closure::wrap(Box::new(func) as Box<dyn Fn(_) -> Promise>);
    browser.runtime().on_message().add_listener(&closure);
    closure.forget();
}

#[derive(Serialize, Deserialize)]
struct Popup {
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub height: u8,
    pub width: u8,
}
