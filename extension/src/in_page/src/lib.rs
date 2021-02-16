extern crate console_error_panic_hook;
use futures::{channel::mpsc, StreamExt};
use js_sys::{global, Object, Promise};
use serde::Deserialize;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local};
use web_sys::MessageEvent;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("IPS: Hello World");

    let global: Object = global();

    let boxed = Box::new(call_backend) as Box<dyn Fn(String) -> Promise>;
    let add_closure = Closure::wrap(boxed);
    let add_closure = add_closure.into_js_value();

    js_sys::Reflect::set(&global, &JsValue::from("call_backend"), &add_closure).unwrap();
}

#[wasm_bindgen]
pub fn call_backend(txt: String) -> Promise {
    let js_value = JsValue::from(txt);

    let (mut sender, mut receiver) = mpsc::channel::<JsValue>(10);
    // create listener
    let func = move |msg: MessageEvent| {
        let js_value: JsValue = msg.data();

        let message: Result<Message, _> = js_value.into_serde();
        if let Ok(Message { target, data }) = message {
            if target != "in-page" {
                return;
            }

            log::info!("IPS: Received response from CS: {:?}", data);
            sender.try_send(JsValue::from_str(&data)).unwrap();
        }
    };

    let closure = Closure::wrap(Box::new(func) as Box<dyn FnMut(_)>);

    // TODO remove event listener again
    window.add_event_listener("message".to_string(), &closure);

    window.post_message(js_value);
    closure.forget();

    let fut = async move {
        let response = receiver.next().await;
        response.ok_or_else(|| JsValue::from_str("IPS: No response from CS"))
    };

    return future_to_promise(fut);
}

#[derive(Deserialize)]
struct Message {
    data: String,
    target: String,
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
        closure: &Closure<dyn FnMut(MessageEvent)>,
    ) -> JsValue;
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
