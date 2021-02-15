use js_sys::{Function, Promise};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MessageEvent;

#[wasm_bindgen]
extern "C" {
    pub type Browser;
    pub static browser: Browser;

    #[wasm_bindgen(method, getter)]
    pub fn runtime(this: &Browser) -> Runtime;
}

#[wasm_bindgen]
extern "C" {
    pub type Runtime;

    #[wasm_bindgen(method, js_name = getURL)]
    pub fn get_url(this: &Runtime, path: String) -> String;

    #[wasm_bindgen(method, js_name = sendMessage)]
    pub fn send_message(this: &Runtime, value: JsValue) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type Event;

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &Event, callback: &Function);

}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &Window, event: String, callback: &Function) -> JsValue;
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from Content Script");

    let rs_window = web_sys::window().expect("no global `window` exists");
    let document = rs_window
        .document()
        .expect("should have a document on window");
    let head = document.head().expect("document should have a body");

    let url = browser.runtime().get_url("js/in_page.js".to_string());

    // Create new script tag
    let script_tag = document.create_element("script").unwrap();
    script_tag.set_attribute("src", &url).unwrap();
    script_tag.set_attribute("type", "module").unwrap();

    // add script to the top
    let first_child = head.first_child();
    head.insert_before(&script_tag, first_child.as_ref())
        .unwrap();

    // create listener
    let cb = Closure::wrap(Box::new(|msg: MessageEvent| {
        let js_value: JsValue = msg.data();
        let string = js_value.as_string().unwrap();
        log::info!("Received message from in-page script {:?}", string);

        browser.runtime().send_message(js_value);
    }) as Box<dyn Fn(_)>);
    window.add_event_listener("message".to_string(), cb.as_ref().unchecked_ref());

    cb.forget();
    Ok(())
}

#[derive(Deserialize)]
struct Message {
    pub data: String,
}
