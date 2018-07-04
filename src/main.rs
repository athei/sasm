extern crate yew;
extern crate sasm;

use yew::prelude::*;
use sasm::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
