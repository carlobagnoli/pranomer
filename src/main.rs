extern crate rustbox;

mod kanban;

use kanban::{app, event, fileio};
use std::default::Default;
use rustbox::RustBox;

fn main() -> std::io::Result<()>{
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "kanban.md"
    };

    let mut app: app::App = fileio::read_kanban_from_file(path).unwrap_or(app::App::new());

    event::main_loop(&rustbox, &mut app);

    fileio::output_contents_to_file(path, &mut app)?;

    Ok(())
}
