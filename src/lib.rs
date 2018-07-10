#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use std::collections::HashMap;

use stdweb::unstable::*;
use stdweb::*;

use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Model>,
    state: State,
    simc: Option<Value>,
    profile: String,
}

enum State {
    Unloaded,
    Loading,
    Idle,
    Simulating,
}

pub enum Msg {
    Button,
    SimDone,
    ProfileUpdate(String),
    WindowEvent(HashMap<String, Value>),
}

fn receive_message(js_event: Value, clb: &Callback<HashMap<String, Value>>) {
    let event = match HashMap::<String, Value>::try_from(js_event) {
        Ok(map) => map,
        _ => return,
    };
    clb.emit(event);
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model = Model { link, simc: None, state: State::Unloaded, profile: "".into() };

        // we click the load button for the user on startup
        model.link.send_back(|_| Msg::Button).emit(());

        // Map a window event to a Msg
        let send_window_event = model.link.send_back(|event: HashMap<String, Value>| Msg::WindowEvent(event));
        let closure = move |e: Value| {
            receive_message(e, &send_window_event);
        };
        js! {
            window.addEventListener("message", function (e) {
                if (e.origin != window.origin)
                    return;
                @{closure}(e.data);
            });
        }

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SimDone => self.state.sim_done(),
            Msg::ProfileUpdate(profile) => {
                self.profile = profile;
                false
            }
            Msg::Button => {
                if !self.state.button_press() {
                    return false;
                }
                match self.state {
                    State::Loading => {
                        self.simc = Some(js! { return Simc() })
                    },
                    State::Simulating => {
                        js! {
                            var ptr = @{&self.simc}.allocateUTF8(@{&self.profile});
                            @{&self.simc}._simulate(ptr);
                            @{&self.simc}._free(ptr);
                        }
                        self.link.send_back(|_| Msg::SimDone).emit(());
                    },
                    _ => ()
                }
                true
            },
            Msg::WindowEvent(event) => {
                let name = match event.get("event").and_then(|v| v.as_str()) {
                    Some(event) => event,
                    _ => return false,
                };
                match name {
                    "simc_loaded" => self.state.engine_loaded(),
                    _ => false,
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
