mod string_parser;
mod parsers;
mod brew_command;
mod tap_command;

use nom::character::complete::{alphanumeric1, multispace0, space0};
use nom::multi::many0;
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

use string_parser::string;
use parsers::*;

#[derive(Debug)]
enum Command<'a> {
    Brew(brew_command::BrewCommand<'a>),
    Tap(tap_command::TapCommand<'a>),
}

fn parse_command(input: &str) -> IResult<&str, Command>{
    // Commands should always be followed by a space
    let (remainder, command) = terminated(
        alphanumeric1,
        space0
    )
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

fn parse_line(input: &str) -> IResult<&str, Command> {
    let (remainder, result) = delimited(
        multispace0,
        parse_command,
        multispace0,
    )(input)?;

    Ok((remainder, result))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_line)(input)
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

