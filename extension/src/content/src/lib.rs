use js_sys::{Function, Object, Promise};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

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

    #[wasm_bindgen(method, js_name = sendMessage)]
    pub fn send_message(
        this: &Runtime,
        extension_id: Option<&str>,
        message: &JsValue,
        options: Option<&Object>,
    ) -> Promise;
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
    log::info!("Hello World from Content Script");

    let value = JsValue::from("Hello World");
    let future = browser.runtime().send_message(None, &value, None);
    spawn(async move {
        let _ = JsFuture::from(future).await?;
        Ok(())
    });
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
