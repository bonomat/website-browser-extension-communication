use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Sign,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    data: String,
    target: String,
    source: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Sign => {
                let msg = Message {
                    data: "signed".to_string(),
                    target: "background".to_string(),
                    source: Some("popup".to_string()),
                };
                let js_value = JsValue::from_serde(&msg).unwrap();
                let _resp = browser.runtime().send_message(js_value);
                // TODO: handle response
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div>
            <p>{ "Hello worlds!" }</p>
                <button onclick=self.link.callback(|_| Msg::Sign)>{ "Sign" }</button>
          </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

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

    #[wasm_bindgen(method, js_name = getBackgroundPage)]
    pub fn get_background_page(this: &Runtime) -> Background;

    #[wasm_bindgen(method, js_name = sendMessage)]
    pub fn send_message(this: &Runtime, value: JsValue) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type Background;

    #[wasm_bindgen(method)]
    pub fn is_locked(this: &Background) -> bool;
}
