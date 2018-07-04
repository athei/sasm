#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    value: u32
}

pub enum Msg {
    Input,
}

impl Component<Self> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Self, Self>) -> Self {
        Model { value: 3 }
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<Self, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Self, Self> for Model {
    fn view(&self) -> Html<Self, Self> {
        html! {
            <div>
                    <button onclick=|_| Msg::Input,>{ "Clear Database" }</button>
            </div>
        }
    }
}