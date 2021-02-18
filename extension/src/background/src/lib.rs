use js_sys::{Array, Object};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_extension::{browser, QueryObject, Tab};
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("BS: Hello World");

    // TODO: Forward tab ID of source to PS, so that we can return the reponse back to the CS
    // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onMessage#addlistener_syntax
    let handle_msg_from_cs = Closure::wrap(Box::new(|js_value: JsValue| {
        if !js_value.is_object() {
            let log = format!("Invalid request: {:?}", js_value);
            log::error!("{}", log);
            return;
        }

        let msg: Message = js_value.into_serde().unwrap();
        if msg.target != "background" || msg.source != "content" {
            log::debug!("Unexpected message: {:?}", msg);
            return;
        }

        log::info!("BS: Received from CS: {:?}", &msg);

        let popup = Popup {
            url: "popup.html".to_string(),
            type_: "popup".to_string(),
            height: 200,
            width: 200,
        };
        let js_value = JsValue::from_serde(&popup).unwrap();
        let object = Object::try_from(&js_value).unwrap();
        let popup_window = browser.windows().create(&object);

        log::info!("Popup created {:?}", popup_window);
    }) as Box<dyn Fn(_)>);
    browser
        .runtime()
        .on_message()
        .add_listener(handle_msg_from_cs.as_ref().unchecked_ref());
    handle_msg_from_cs.forget();

    // receiving transaction to be signed and published
    let handle_msg_from_ps = Closure::wrap(Box::new(move |js_value: JsValue| {
        if !js_value.is_object() {
            let log = format!("Invalid request: {:?}", js_value);
            log::error!("{}", log);
            return;
        }

        let msg: Message = js_value.into_serde().unwrap();
        if msg.target != "background" || msg.source != "popup" {
            log::debug!("Unexpected message: {:?}", msg);
            return;
        }

        log::info!("Received message from Popup: {:?}", msg);

        spawn_local(async {
            let query_object = QueryObject {
                current_window: false,
                active: true,
            };
            let tabs = browser
                .tabs()
                .query(Object::try_from(&JsValue::from_serde(&query_object).unwrap()).unwrap());
            let tabs = JsFuture::from(tabs).await.unwrap();

            // TODO: Know the tab ID of the tab where the content script was injected beforehand
            let tabs: Array = tabs.into();
            let tab: Tab = tabs.find(&mut |_, _, _| true).unchecked_into();
            let _resp = browser.tabs().send_message(
                tab.id(),
                JsValue::from_serde(&Message {
                    data: msg.data,
                    target: "content".to_string(),
                    source: "background".to_string(),
                })
                .unwrap(),
                JsValue::null(),
            );
        });

        // TODO: Inform PS about success/failure
        // Promise::resolve(&JsValue::from_str("ACK2"))
    }) as Box<dyn FnMut(JsValue)>);

    browser
        .runtime()
        .on_message()
        .add_listener(handle_msg_from_ps.as_ref().unchecked_ref());
    handle_msg_from_ps.forget();
}

#[derive(Serialize, Deserialize)]
struct Popup {
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub height: u8,
    pub width: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    data: String,
    target: String,
    source: String,
}

#[derive(Debug, Deserialize)]
struct PopupWindow {
    id: u16,
}
