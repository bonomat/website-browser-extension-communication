use js_sys::{Function, Object, Promise};
use serde::Deserialize;
use serde::Serialize;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

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
    pub fn add_listener(this: &Event, callback: &Function);

}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from Background Script");

    let closure = Closure::wrap(Box::new(|msg: JsValue| {
        log::info!("Received: {}", msg.as_string().unwrap());
        let popup = Popup {
            url: "popup.html".to_string(),
            type_: "popup".to_string(),
            height: 200,
            width: 200,
        };
        let js_value = JsValue::from_serde(&popup).unwrap();
        let object = Object::try_from(&js_value).unwrap();
        let _x = browser.windows().create(&object);
    }) as Box<dyn FnMut(_)>);

    browser
        .runtime()
        .on_message()
        .add_listener(closure.as_ref().unchecked_ref());

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
