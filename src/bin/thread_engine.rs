extern crate yew;
extern crate sasm;

use yew::prelude::*;
use sasm::engine::Engine;

fn main() {
    yew::initialize();
    Engine::register();
    yew::run_loop();
}