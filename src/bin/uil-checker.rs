#![feature(path)]
#![feature(io)]

extern crate client;

use std::old_io::{File, BufferedReader};
use client::ui;

fn main() {
    let file = File::open(&Path::new("assets/markup/test.xml")).unwrap();
    let reader = BufferedReader::new(file);

    let mut parser = ui::markup::Parser::new(ui::StdOutErrorReporter, reader);

    parser.parse();
}
