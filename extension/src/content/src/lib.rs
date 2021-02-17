use js_sys::Promise;
use serde::{Deserialize, Serialize};
use std::future::Future;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_extention::browser;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::MessageEvent;

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
    let func = |msg: JsValue| {
        let msg: MessageEvent = msg.into();
        let js_value: JsValue = msg.data();
        // TODO: Actually only accept messages from IPS
        if let Some(string) = js_value.as_string() {
            log::info!("CS: Received from IPS: {:?}", string);
            let msg = Message {
                data: string,
                target: "background".to_string(),
                source: Some("content".to_string()),
            };
            // sending message to Background script
            let js_value = JsValue::from_serde(&msg).unwrap();
            let resp: Promise = browser.runtime().send_message(js_value);
            spawn(async {
                let rs_window = web_sys::window().expect("no global `window` exists");
                let resp = JsFuture::from(resp).await?;
                log::info!("CS: Received response from BS: {:?}", resp);

                rs_window.post_message(
                    &JsValue::from_serde(&Message {
                        data: resp.as_string().unwrap(),
                        target: "in-page".to_string(),
                        source: Some("content".to_string()),
                    })
                    .unwrap(),
                    "*",
                )?;
                Ok(())
            });
        }
    };

    let cb = Closure::wrap(Box::new(func) as Box<dyn Fn(_)>);
    rs_window
        .add_event_listener_with_callback("message", cb.as_ref().unchecked_ref())
        .unwrap();

    cb.forget();

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Message {
    data: String,
    target: String,
    source: Option<String>,
}

pub async fn unwrap_future<F>(future: F)
where
    F: Future<Output = Result<(), JsValue>>,
{
    if let Err(e) = future.await {
        log::error!("{:?}", &e);
    }
}

pub fn spawn<A>(future: A)
where
    A: Future<Output = Result<(), JsValue>> + 'static,
{
    spawn_local(unwrap_future(future))
}
