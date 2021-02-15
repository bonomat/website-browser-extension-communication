use js_sys::Function;
use wasm_bindgen::prelude::*;

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
}

#[wasm_bindgen]
extern "C" {
    pub type Event;

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &Event, callback: &Function);

}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from Content Script");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
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
    Ok(())
}
