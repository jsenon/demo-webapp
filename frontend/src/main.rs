extern crate yew;
extern crate frontend;

use yew::prelude::*;
use frontend::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}