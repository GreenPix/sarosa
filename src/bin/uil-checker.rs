#![feature(path)]
#![feature(io)]

extern crate client;

use std::old_io::{File, BufferedReader};
use client::ui;

fn main() {
    {
        let file = File::open(&Path::new("assets/markup/test.xml")).unwrap();
        let reader = BufferedReader::new(file);

        ui::markup::parse(ui::StdOutErrorReporter, reader);
    }
    let styledefs = {
        let file = File::open(&Path::new("assets/markup/test.xml")).unwrap();
        let reader = BufferedReader::new(file);

        ui::deps::parse(ui::StdOutErrorReporter, reader)
    };
    {
        let file = File::open(&Path::new("assets/style/test.style")).unwrap();
        let reader = BufferedReader::new(file);

        ui::style::parse(ui::StdOutErrorReporter, reader, &styledefs);
    }
}
