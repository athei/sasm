extern crate yew;
extern crate sasm;

use yew::prelude::*;
use sasm::Model;

type Mine = App<Model, Model>;


fn main() {
    let model = Model::create((), Env<Model, Model>::new());
    yew::initialize();
    yew::run_loop();
}
