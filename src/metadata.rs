use nom::branch::alt;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, IResult};
use nom::character::complete::{alpha1, space1};

use crate::brew_command;
use crate::metafield::Metafield;
use crate::tap_command;

#[derive(Debug)]
enum Command<'a> {
    Brew(brew_command::BrewCommand<'a>),
    Tap(tap_command::TapCommand<'a>),
    None,
}

#[derive(Debug)]
pub struct MetaCommand<'a> {
    description: &'a str,
    optional: bool,
    command: Command<'a>,
}


fn parse_metacommand(input: &str) -> IResult<&str, MetaCommand> {

    let mut metacommand: MetaCommand = MetaCommand {
        description: "",
        optional: false,
        command: Command::None,
    };

    let mut res_remainder = input;
    let mut sentinel = true;
    while !res_remainder.is_empty() && sentinel {
        let (remainder, command) = alt((
            terminated(alpha1, space1),
            terminated(tag("##"), space1),
        ))(res_remainder.trim())?;

        res_remainder = match command {
            "tap" => {
                let (remainder, tap) = tap_command::TapCommand::parse(remainder)?;
                metacommand.command = Command::Tap(tap);
                sentinel = false;
                remainder
            }
            "brew" => {
                let (remainder, brew) = brew_command::BrewCommand::parse(remainder)?;
                metacommand.command = Command::Brew(brew);
                sentinel = false;
                remainder
            }
            "##" => {
                let (remainder, metadata) = Metafield::parse(remainder)?;
                match metadata {
                    Metafield::Optional => metacommand.optional = true,
                    Metafield::Description(value) => metacommand.description = value,
                };
                remainder
            }
            unknown => panic!("Unknown command: {unknown} - {remainder}"),
        };
    };

    Ok((res_remainder, metacommand))
}

pub fn parse_command(input: &str) -> IResult<&str, Vec<MetaCommand>> {
    let mut input = input.trim();

    let mut commands: Vec<MetaCommand> = Vec::new();
    while !input.is_empty() {
        let (remainder, metacommand) = parse_metacommand(input)?;
        input = remainder;
        commands.push(metacommand);
    };
    Ok((input, commands))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_line() {
//         let (remainder, brew) = BrewCommand::parse("\"package\" \n").unwrap();
//         assert_eq!(brew.pkg, "package");
//         assert_eq!(remainder, "");
//
//         // Remainder must be after the line break
//         let (remainder, brew) = BrewCommand::parse("\"package\" \nextra").unwrap();
//         assert_eq!(brew.pkg, "package");
//         assert_eq!(remainder, "extra");
//
//         // Returns an error when following invalid input
//         let res = BrewCommand::parse("\"package\", invalid: true\nextra");
//         match res {
//             Err(nom::Err::Error(err)) => {
//                 assert_eq!(err.to_string(), "error Tag at: invalid: true\nextra"); // Checking the remaining input
//             }
//             _ => panic!("Expected an error but got: {:?}", res),
//         }
//     }
//
//     #[test]
//     fn parse_args() {
//         let (remainder, brew) = BrewCommand::parse("\"pkg\", args: [\"hello\", \"world\"]\n").unwrap();
//         assert_eq!(brew.args, vec!["hello", "world"]);
//         assert_eq!(remainder, "");
//
//         let (remainder, brew) = BrewCommand::parse("\"pkg\", args: {\"hello\": \"world\"}\n").unwrap();
//         assert_eq!(brew.args, vec!["hello", "world"]);
//         assert_eq!(remainder, "");
//
//         // Should only accept maps or lists
//         let res = BrewCommand::parse("\"pkg\", args: hello\n");
//         match res {
//             Err(nom::Err::Error(err)) => {
//                 assert_eq!(err.to_string(), "error Tag at: hello\n"); // Checking the remaining input
//             }
//             _ => panic!("Expected an error but got: {:?}", res),
//         }
//
//         // Should fail on dangling args
//         let res = BrewCommand::parse("\"pkg\", args: \n");
//         match res {
//             Err(nom::Err::Error(err)) => {
//                 assert_eq!(err.to_string(), "error Tag at: \n"); // Checking the remaining input
//             }
//             _ => panic!("Expected an error but got: {:?}", res),
//         }
//     }
//
//     #[test]
//     fn parse_link() {
//         let (remainder, brew) = BrewCommand::parse("\"package\", link: true \n").unwrap();
//         assert_eq!(brew.pkg, "package");
//         assert_eq!(brew.link, LinkOptions::On);
//         assert_eq!(remainder, "");
//
//         let (remainder, brew) = BrewCommand::parse("\"package\", link: :override").unwrap();
//         assert_eq!(brew.pkg, "package");
//         assert_eq!(brew.link, LinkOptions::Override);
//         assert_eq!(remainder, "");
//     }
// }

