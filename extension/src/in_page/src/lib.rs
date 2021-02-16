use futures::channel::oneshot;
use futures::TryFutureExt;
use js_sys::{global, Promise};
use js_sys::{Function, Object};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::MessageEvent;

#[wasm_bindgen(start)]
pub fn main() {
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

    let (sender, receiver) = oneshot::channel::<JsValue>();
    // create listener
    let func = move |msg: MessageEvent| {
        if msg.origin() == localhost
            return;
        let js_value: JsValue = msg.data();
        let string = js_value.as_string().unwrap();
        log::info!("IPS: Received from {:?}: {:?}", msg.origin(), string);

        // callback(string);
        sender.send(JsValue::from_str(&string));
        return Promise::resolve(&JsValue::from(""));
    };

    let closure = Closure::once(func);

    // TODO remove event listener again
    window.add_event_listener("message".to_string(), &closure);

    window.post_message(js_value);
    closure.forget();

    return future_to_promise(
        receiver
            .inspect_ok(|resp| {
                log::info!("IPS: Received response from CS: {:?}", resp);
            })
            .map_err(|er| JsValue::from(er.to_string())),
    );
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Window, value: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(
        this: &Window,
        event: String,
        // TODO: do we have to use MessageEvent here?
        closure: &Closure<dyn FnMut(MessageEvent) -> Promise>,
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
