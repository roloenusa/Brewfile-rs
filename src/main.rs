mod string_parser;
mod parsers;
mod brew_command;
mod tap_command;

use nom::character::complete::{alpha1, space1};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, Parser};

use parsers::*;

#[derive(Debug)]
enum Command<'a> {
    Brew(brew_command::BrewCommand<'a>),
    Tap(tap_command::TapCommand<'a>),
}

fn parse_command(input: &str) -> IResult<&str, Command>{
    // Commands should always be followed by a space
    let (remainder, command) = terminated(alpha1, space1)
    .parse(input)?;

    match command {
        "tap" => {
            let (remainder, tap) = tap_command::TapCommand::parse(remainder)?;
            Ok((remainder, Command::Tap(tap)))
        }
        "brew" => {
            let (remainder, brew) = brew_command::BrewCommand::parse(remainder)?;
            Ok((remainder, Command::Brew(brew)))
        }
        unknown => panic!("Unknown command: {unknown} - {remainder}"),
    }
}


fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_line(parse_command))(input)
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

