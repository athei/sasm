#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod engine;
use yew::prelude::*;

pub struct Model {
    state: State,
    profile: String,
    engine: Box<Bridge<engine::Engine>>,
}

enum State {
    Loading,
    Idle,
    Simulating,
}

pub enum Msg {
    Loaded,
    Button,
    SimDone,
    ProfileUpdate(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|response| {
            match response {
                engine::Response::LoadDone => Msg::Loaded,
                engine::Response::SimulationDone(_) => Msg::SimDone,
            }
        });
        let engine = engine::Engine::bridge(callback);
        let mut model = Model { state: State::Loading, profile: "".into(), engine };

        model.engine.send(engine::Request::Load);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded => self.state.engine_loaded(),
            Msg::SimDone => self.state.sim_done(),
            Msg::ProfileUpdate(profile) => {
                self.profile = profile;
                false
            },
            Msg::Button => {
                if !self.state.button_press() {
                    return false;
                }
                match self.state {
                    State::Simulating => {
                        self.engine.send(engine::Request::Simulate(self.profile.clone()));
                        true
                    }
                    _ => false
                }
            },
        }
    }
}

impl Renderable<Self> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <textarea placeholder="Enter simc profile.", rows="30", cols="50", oninput=|e| Msg::ProfileUpdate(e.value),></textarea>
                <button disabled=self.state.button_disabled(), onclick=|_| Msg::Button,>{ self.state.button_text() }</button>
            </div>
        }
    }
}

impl State {
    fn button_text(&self) -> &str {
        match self {
            State::Loading => "Loading Engine...",
            State::Idle => "Start Simulation",
            State::Simulating => "Simulating...",
        }
    }

    fn button_press(&mut self) -> bool {
        match self {
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

    fn sim_done(&mut self) -> bool {
        match self {
            State::Simulating => {
                *self = State::Idle;
                true
            },
            _ => false,
        }
    }
}
