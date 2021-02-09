use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {
    msg: String,
}

pub enum Msg {}

#[wasm_bindgen(module = "/src/js/popup_script.js")]
extern "C" {
    fn postMessage();
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        postMessage();
        let msg = "hello world".to_string();
        App { msg }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p>{ self.msg.clone() }</p>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
