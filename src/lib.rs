#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    value: u32
}

pub enum Msg {
    Input,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: 3 }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Self> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                    <button onclick=|_| Msg::Input,>{ "Clear Database" }</button>
            </div>
        }
    }
}