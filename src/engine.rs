use std::collections::HashMap;
use stdweb::unstable::*;
use stdweb::*;
use yew::prelude::worker::*;
use yew::prelude::Callback;

pub struct Engine {
    link: AgentLink<Engine>,
    simc: Option<Value>,
    owner: Option<HandlerId>,
    loaded: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    Load,
    Simulate(String)
}
impl Transferable for Request {}

#[derive(Serialize, Deserialize)]
pub enum Response {
    LoadDone,
    SimulationDone(String),
}
impl Transferable for Response {}

pub enum Msg {
    WindowMessage(HashMap<String, Value>),
}

fn receive_message(js_event: Value, clb: &Callback<HashMap<String, Value>>) {
    let event = match HashMap::<String, Value>::try_from(js_event) {
        Ok(map) => map,
        _ => return,
    };
    clb.emit(event);
}

impl Agent for Engine {
    type Reach = Job;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let send_window_event = link.send_back(|event: HashMap<String, Value>| Msg::WindowMessage(event));
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
        Engine { link, simc: None, owner: None, loaded: false }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::WindowMessage(msg) => {
                let name = match msg.get("event").and_then(|v| v.as_str()) {
                    Some(msg) => msg,
                    _ => return,
                };
                match name {
                    "simc_loaded" => {
                        self.loaded = true;
                        self.link.response(self.owner.unwrap(), Response::LoadDone);
                    }
                    _ => (),
                }
            },
        }
    }

    fn handle(&mut self, input: Self::Input, id: HandlerId) {
        match input {
            Request::Load => {
                assert!(self.simc.is_none());
                self.simc = Some(js! { return Simc() });
                self.owner = Some(id);
            }
            Request::Simulate(profile) => {
                assert!(self.loaded);
                js! {
                    var ptr = @{&self.simc}.allocateUTF8(@{&profile});
                    @{&self.simc}._simulate(ptr);
                    @{&self.simc}._free(ptr);
                }
                self.link.response(id, Response::SimulationDone("123".into()));
            }
        }
    }
}
