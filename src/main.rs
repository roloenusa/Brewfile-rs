mod string_parser;
mod parsers;
mod brew_command;
mod tap_command;
mod metadata;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space1};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, Parser};

use parsers::*;

#[derive(Debug)]
enum Command<'a> {
    Brew(brew_command::BrewCommand<'a>),
    Tap(tap_command::TapCommand<'a>),
    Metadata(metadata::Metadata<'a>),
}

fn parse_command(input: &str) -> IResult<&str, Vec<Command>> {
    let input = input.trim();

    // If empty, then everything is compiled. Return
    if input.is_empty() {
        let result: Vec<Command> = Vec::new();
        return Ok((input, result))
    }

    let (remainder, command) = alt((
        terminated(alpha1, space1),
        terminated(tag("##"), space1),
    ))(input)?;

    let (remainder, command) = match command {
        "tap" => {
            let (remainder, tap) = tap_command::TapCommand::parse(remainder)?;
            (remainder, Command::Tap(tap))
        }
        "brew" => {
            let (remainder, brew) = brew_command::BrewCommand::parse(remainder)?;
            (remainder, Command::Brew(brew))
        }
        "##" => {
            let (remainder, metadata) = metadata::Metadata::parse(remainder)?;
            (remainder, Command::Metadata(metadata))
        }
        unknown => panic!("Unknown command: {unknown} - {remainder}"),
    };

    let (remainder, mut commands) = parse_command(remainder)?;
    let last = commands.last_mut();

    match command {
        Command::Metadata(m) => {
            match m {
                metadata::Metadata::Optional if last.is_some() => {
                    if let Command::Tap(s) = last.expect("Value should be here") {
                        s.optional = true;
                    }
                },
                metadata::Metadata::Description(value) if last.is_some() => {
                    if let Command::Tap(s) = last.expect("Value should be here") {
                        s.description = value;
                    }
                },
                _ => (),
            };
        },
        _ => {
            commands.push(command);
        }
    };

    Ok((remainder, commands))
}

// fn parse_command_rec(input: &str, chain) -> IResult<&str, Command>{
//     // Commands should always be followed by a space
//     let (remainder, command) = alt((
//         terminated(alpha1, space1),
//         terminated(tag("##"), space1),
//     ))
//     .parse(input)?;
//
//     match command {
//         "tap" => {
//             let (remainder, tap) = tap_command::TapCommand::parse(remainder)?;
//             Ok((remainder, Command::Tap(tap)))
//         }
//         "brew" => {
//             let (remainder, brew) = brew_command::BrewCommand::parse(remainder)?;
//             Ok((remainder, Command::Brew(brew)))
//         }
//         "##" => {
//             let (remainder, metadata) = metadata::Metadata::parse(remainder)?;
//             Ok((remainder, Command::Metadata(metadata.clone())))
//         }
//         unknown => panic!("Unknown command: {unknown} - {remainder}"),
//     }
// }


fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    // many0(parse_line(parse_command))(input)
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

