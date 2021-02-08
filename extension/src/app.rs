use yew::prelude::*;

pub struct App {
    msg: String
}

pub enum Msg {}

#[wasm_bindgen(module = "background_script.js")]
extern "C" {
    fn getMsg() -> String;
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {

        App { msg: getMsg() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Hello World!" }</p>
        }
    }

    fn rendered(&mut self, _first_render: bool) {

    }

    fn destroy(&mut self) {

    }
}
