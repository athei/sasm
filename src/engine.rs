use stdweb::*;
use yew::prelude::worker::*;

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
    LoadDone
}

impl Agent for Engine {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn name_of_resource() -> &'static str { "thread_engine.js" }

    fn create(link: AgentLink<Self>) -> Self {
        let send_loaded = link.send_back(|_| Msg::LoadDone);
        let closure = move || {
            send_loaded.emit(());
        };
        js! {
            self.simc_callbacks = {
                "loaded": function(e) {
                    @{closure}();
                }
            };
            importScripts("engine.js");
        }
        Engine { link, simc: None, owner: None, loaded: false }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::LoadDone => {
                self.loaded = true;
                self.link.response(self.owner.unwrap(), Response::LoadDone);
            },
        };
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
