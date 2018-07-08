#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use stdweb::Value;

use yew::prelude::*;

pub struct Model {
    state: State,
    simc: Option<Value>,
}

enum State {
    Unloaded,
    Loading,
    Idle,
    Simulating,
}

pub enum Msg {
    Button,
    Loaded,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { simc: None, state: State::Unloaded }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Button => {
                if !self.state.button_press() {
                    return false;
                }
                match self.state {
                    State::Loading => {
                        self.simc = Some(js! { return Simc() })
                    },
                    _ => ()
                }
                true
            },
            Msg::Loaded => self.state.engine_loaded(),
        }
    }
}

impl Renderable<Self> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <textarea rows="30", cols="50",>{ "BLUB" }</textarea>
                <button disabled=self.state.button_disabled(), onclick=|_| Msg::Button,>{ self.state.button_text() }</button>
                <a id="engine_loaded", onclick=|_| Msg::Loaded,></a>
            </div>
        }
    }
}

impl State {
    fn button_text(&self) -> &str {
        match self {
            State::Unloaded => "Load Engine",
            State::Loading => "Loading Engine...",
            State::Idle => "Start Simulation",
            State::Simulating => "Simulating...",
        }
    }

    fn button_press(&mut self) -> bool {
        match self {
            State::Unloaded => {
                *self = State::Loading;
                true
            },
            State::Idle => {
                *self = State::Simulating;
                true
            },
            _ => false
        }
    }

    fn button_disabled(&self) -> bool {
        match self {
            State::Loading => true,
            State::Simulating => true,
            _ => false,
        }
    }

    fn engine_loaded(&mut self) -> bool {
        match self {
            State::Loading => {
                *self = State::Idle;
                true
            },
            _ => false,
        }
    }
}