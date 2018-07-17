use stdweb::*;
use stdweb::unstable::*;
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
    ProgressUpdate(Progress),
    SimulationDone(String),
}
impl Transferable for Response {}

pub enum Msg {
    LoadDone,
    ProgressUpdate(Progress),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
    iteration: u32,
    total_iterations: u32,
    phase: u32,
    total_phases: u32,
    phase_name: String,
    subphase_name: String,
}
js_deserializable!(Progress);

impl Agent for Engine {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn name_of_resource() -> &'static str { "thread_engine.js" }

    fn create(link: AgentLink<Self>) -> Self {
        let send_loaded = link.send_back(|_| Msg::LoadDone);
        let send_progress = link.send_back(|p| Msg::ProgressUpdate(p));
        let closure_loaded = move || {
            send_loaded.emit(());
        };
        let closure_progress = move |p: Value| {
            let progress = p.try_into().unwrap();
            send_progress.emit(progress);
        };
        js! {
            self.simc_callbacks = {
                "loaded": function() {
                    @{closure_loaded}();
                },
                "update_progress": function(progress) {
                    @{closure_progress}(progress)
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
            Msg::ProgressUpdate(progress) => {
                self.link.response(self.owner.unwrap(), Response::ProgressUpdate(progress))
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
                let result = js! {
                    var ptr_in = @{&self.simc}.allocateUTF8(@{&profile});
                    var ptr_out = @{&self.simc}._simulate(ptr_in);
                    @{&self.simc}._free(ptr_in);
                    var result = @{&self.simc}.UTF8ToString(ptr_out);
                    @{&self.simc}._free(ptr_out);
                    return result;
                };
                self.link.response(id, Response::SimulationDone(result.try_into().unwrap()));
            }
        }
    }
}
