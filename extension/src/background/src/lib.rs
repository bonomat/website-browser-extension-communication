use js_sys::{Object, Promise};
use serde::Deserialize;
use serde::Serialize;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method)]
    pub fn open(this: &Window, url: String) -> Window;
}

#[wasm_bindgen]
extern "C" {
    pub type Browser;
    pub static browser: Browser;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_action(this: &Browser) -> Sidebar;

    #[wasm_bindgen(method, getter)]
    pub fn windows(this: &Browser) -> Windows;

}

#[wasm_bindgen]
extern "C" {
    pub type Sidebar;

    #[wasm_bindgen(method, getter)]
    pub fn open(this: &Sidebar) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type Windows;

    #[wasm_bindgen(method)]
    pub fn create(this: &Windows, info: &Object) -> Promise;
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from Background Script");

    let popup = Popup {
        url: "popup.html".to_string(),
        type_: "popup".to_string(),
        height: 200,
        width: 200,
    };
    let js_value = JsValue::from_serde(&popup).unwrap();

    log::info!("Are you available js_value? {:?}", js_value.is_object());
    let object = Object::try_from(&js_value).unwrap();
    log::info!("Are you available object? {:?}", object);
    let x = browser.windows().create(&object);
    log::info!("Are you available window ? {:?}", x);

    // let future = browser.sidebar_action().open();
    // spawn(async move {
    //     let _ = JsFuture::from(future).await?;
    //     Ok(())
    // });
}

#[derive(Serialize, Deserialize)]
struct Popup {
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub height: u8,
    pub width: u8,
}

// pub fn unwrap_future<F>(future: F) -> impl Future<Output = ()>
//     where F: Future<Output = Result<(), JsValue>> {
//     async {
//         if let Err(e) = future.await {
//             log::error!("{:?}",&e);
//         }
//     }
// }
//
//
// pub fn spawn<A>(future: A) where A: Future<Output = Result<(), JsValue>> + 'static {
//     spawn_local(unwrap_future(future))
// }
