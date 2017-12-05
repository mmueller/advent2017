extern crate argparse;
extern crate itertools;

mod days;

use argparse::{ArgumentParser, StoreOption};

fn main() {
    let mut day: Option<usize> = None;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Advent of Code 2017");
        parser.refer(&mut day)
              .add_option(&["-d", "--day"], StoreOption,
                          "number of challenge to run");
        parser.parse_args_or_exit();
    }
    match day {
        Some(ref day) => days::solve(*day),
        None => println!("--day is required"),
    }
}
