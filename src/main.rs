extern crate rustbox;

mod agile;

use agile::{event, fileio, Agile};
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

    let mut agile: Agile = fileio::read_kanban_from_file(path).unwrap_or(Agile::new());

    event::main_loop(&rustbox, &mut agile);

    fileio::output_contents_to_file(path, &mut agile)?;

    Ok(())
}
