use futures::{
    channel::{mpsc, oneshot},
    StreamExt,
};
use js_sys::{Function, Object, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
extern "C" {

    #[derive(Debug)]
    pub type Document;

    #[wasm_bindgen(method)]
    pub fn write(this: &Document, content: String);

}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, getter)]
    pub fn document(this: &Window) -> Document;

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
    pub type Windows;

    #[wasm_bindgen(method)]
    pub fn create(this: &Windows, info: &Object) -> Promise;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Windows, window_id: u16) -> Promise;
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
    pub fn add_listener(this: &Event, closure: &Function);

}

#[wasm_bindgen]
pub fn is_locked() -> bool {
    true
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("BS: Hello World");

    let func = |js_value: JsValue| {
        if !js_value.is_object() {
            let log = format!("Invalid request: {:?}", js_value);
            log::error!("{}", log);
            return Promise::resolve(&JsValue::from(log));
        }

        let msg: Message = js_value.into_serde().unwrap();
        if msg.target != "background" {
            return Promise::resolve(&JsValue::from("Invalid target"));
        }

        if let Some(origin) = msg.source.clone() {
            if origin == "popup" {
                log::debug!("Msg received in wrong handler");
                return Promise::resolve(&JsValue::from("Ignored"));
            }
        }

        log::info!("BS: Received from CS: {:?}", &msg);
        let response = JsValue::from("World");
        let popup = Popup {
            url: "popup.html".to_string(),
            type_: "popup".to_string(),
            height: 200,
            width: 200,
        };
        let js_value = JsValue::from_serde(&popup).unwrap();
        let object = Object::try_from(&js_value).unwrap();
        let x = browser.windows().create(&object);

        let (mut sender, mut receiver) = mpsc::channel::<JsValue>(10);

        let mut sender = sender.clone();
        let cb = Closure::wrap(Box::new(move |window_js_value: JsValue| {
            let mut sender = sender.clone();
            let popup_window: PopupWindow = window_js_value.into_serde().unwrap();
            let cb = Closure::wrap(Box::new(move |js_value: JsValue| {
                let mut sender = sender.clone();
                if !js_value.is_object() {
                    let log = format!("Invalid request: {:?}", js_value);
                    log::error!("{}", log);
                    return Promise::resolve(&JsValue::from(log));
                }

                let msg: Message = js_value.into_serde().unwrap();
                if msg.target != "background" {
                    return Promise::resolve(&JsValue::from("Invalid target"));
                }

                // TODO check if msg was from popup
                // TODO use channels to respond to CS
                log::info!("Receveid message from Popup: {:?}", msg);

                sender.try_send(JsValue::from(msg.data)).unwrap();

                return Promise::resolve(&JsValue::from_str("ACK2"));
            }) as Box<dyn FnMut(JsValue) -> Promise>);
            browser
                .runtime()
                .on_message()
                .add_listener(cb.as_ref().unchecked_ref());
            cb.forget();
            log::info!("Popup created {:?}", popup_window);
        }) as Box<dyn FnMut(JsValue) -> _>);
        let _promise = x.then(&cb);
        cb.forget();

        let future = async move {
            let response = receiver.next().await;
            let response = response.ok_or_else(|| JsValue::from_str("IPS: No response from CS"))?;

            Ok(response)
        };

        return future_to_promise(future);
    };
    let closure = Closure::wrap(Box::new(func) as Box<dyn Fn(_) -> Promise>);
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

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    data: String,
    target: String,
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PopupWindow {
    id: u16,
}
