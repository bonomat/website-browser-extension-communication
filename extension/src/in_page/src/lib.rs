use js_sys::Object;
use js_sys::{global, Promise};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("IPS: Hello World");

    let global: Object = global();

    let boxed = Box::new(call_backend) as Box<dyn Fn(String)>;
    let add_closure = Closure::wrap(boxed);
    let add_closure = add_closure.into_js_value();

    js_sys::Reflect::set(&global, &JsValue::from("call_backend"), &add_closure).unwrap();
}

#[wasm_bindgen]
pub fn call_backend(txt: String) {
    let js_value = JsValue::from(txt);
    let future = window.post_message(js_value);

    // let future = promise.clone();
    // log::info!("IPS: Received response from CS: {:?}", response);
    // TODO read from the future
    spawn(async move {
        let resp = JsFuture::from(future).await?;
        log::info!(
            "IPS: Received response from CS: {:?}",
            resp.as_string().is_some()
        );
        Ok(())
    });
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Window, value: JsValue) -> Promise;
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
