mod string_parser;
mod parsers;
mod brew_command;
mod tap_command;
mod metadata;
mod metafield;

use metadata::{parse_command, MetaCommand};
use nom::IResult;

use parsers::*;

fn parse_input(input: &str) -> IResult<&str, Vec<MetaCommand>> {
    parse_command(input)
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{}", src);

    let (remainder, result) = parse_input(&src).unwrap();

    println!("remainder: {:#?}", remainder);

    for command in result {
        println!("{:#?}", command);
    }
}

