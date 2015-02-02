
extern crate client;

use std::old_io::{File, BufferedReader};
use client::ui;

fn main() {
    let file = File::open(&Path::new("assets/markup/test.xml")).unwrap();
    let reader = BufferedReader::new(file);

    let parser = ui::markup::Parser::new(ui::StdOutErrorReporter);

    parser.parse(reader);
}
