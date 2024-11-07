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
    Tap(Tap<'a>),
}

#[derive(Debug, Clone, Default)]
struct Tap<'a> {
    user_repo: &'a str,
    url: Option<&'a str>,
}

fn parse_tap(input: &str) -> IResult<&str, Tap> {
    let (remainder, user_repo) = string::<()>(input).unwrap();
    let (remainder, result) = is_last(remainder)?;

    if result {
        return Ok((remainder, Tap { user_repo, url: None }))
    }

    let (remainder, url) = string::<()>(remainder).unwrap();

    // Build the object we need to return
    Ok((remainder, Tap { user_repo, url: Some(url) }))
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
            let (remainder, tap) = parse_tap(remainder)?;
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

