use js_sys::Promise;
use serde::Serialize;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
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
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Window, value: JsValue);

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(
        this: &Window,
        event: String,
        // TODO: do we have to use MessageEvent here?
        closure: &Closure<dyn Fn(MessageEvent)>,
    ) -> JsValue;
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("CS: Hello World");

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
    let func = |msg: MessageEvent| {
        let js_value: JsValue = msg.data();
        // TODO: Actually only accept messages from IPS
        if let Some(string) = js_value.as_string() {
            log::info!("CS: Received from IPS: {:?}", string);

            let resp: Promise = browser.runtime().send_message(js_value);
            spawn(async move {
                let resp = JsFuture::from(resp).await?;
                log::info!("CS: Received response from BS: {:?}", resp);

                window.post_message(
                    JsValue::from_serde(&Message {
                        data: resp.as_string().unwrap(),
                        target: "in-page".to_string(),
                    })
                    .unwrap(),
                );
                Ok(())
            });
        }
    };

    let cb = Closure::wrap(Box::new(func) as Box<dyn Fn(_)>);
    window.add_event_listener("message".to_string(), &cb);

    cb.forget();

    Ok(())
}

#[derive(Serialize)]
struct Message {
    data: String,
    target: String,
}

pub fn unwrap_future<F>(future: F) -> impl Future<Output = ()>
where
    F: Future<Output = Result<(), JsValue>>,
{
    async {
        if let Err(e) = future.await {
            log::error!("{:?}", &e);
        }
    }
}

pub fn spawn<A>(future: A)
where
    A: Future<Output = Result<(), JsValue>> + 'static,
{
    spawn_local(unwrap_future(future))
}
