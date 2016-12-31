mod cell;
mod space;
mod parse;
mod mem;
mod exec;

use std::env;
use std::io::prelude::*;
use std::fs::File;

use space::Space;
use parse::parse;
use exec::execute;

fn main() {
    let path = env::args().nth(1)
        .expect("Usage: aheui [filename]");
    let mut f = File::open(path).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();

    let space = Space::from_cells(parse(&code));
    execute(space);
}
