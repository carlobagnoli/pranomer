extern crate rustbox;

mod agile;

use agile::{event, fileio, Agile};
use std::default::Default;
use rustbox::RustBox;

fn main() -> std::io::Result<()>
{
    let rustbox = RustBox::init(Default::default())
                          .unwrap_or_else(|e| panic!("{}", e));

    let mut agile: Agile = fileio::read_app_from_folder()
                                  .unwrap_or(Agile::new());

    event::main_loop(&rustbox, &mut agile);

    fileio::output_contents_to_folder(&mut agile)?;

    Ok(())
}
