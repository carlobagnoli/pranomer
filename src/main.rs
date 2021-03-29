extern crate rustbox;

mod kanban;

use kanban::{
    app,
    event
};

use std::default::Default;

use rustbox::RustBox;
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut app: app::App = app::App::new();

    event::main_loop(&rustbox, &mut app);
}
