use nom::bytes::complete::take_until;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, sequence::terminated, IResult};
use nom::character::complete::{alpha1, multispace0, space0, space1};

use crate::is_last;

// #[derive(Debug, Clone)]
// pub struct Metadata<'a> {
//     description: &'a str,
//     required: bool,
// }

#[derive(Debug, Clone)]
pub enum Metadata<'a> {
    Description(&'a str),
    Optional
}

impl<'a> Metadata<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {

        // Check if this is the last parameter on the set
        let (remainder, key) = preceded(tag("@"), alpha1)(input)?;
        match key {
            "description" => {
                let (remainder, value) = take_until("\n")(remainder)?;
                // metadata.description = value.trim();
                // remainder
                Ok((remainder, Metadata::Description(value.trim())))
            },
            "optional" => {
                let (remainder, _) = take_until("\n")(remainder)?;
                // metadata.description = value.trim();
                // remainder
                Ok((remainder, Metadata::Optional))
            },
            unknown => panic!("Unknown parameter {unknown}"),
        }

        // print!("---- remainder: {}", remainder);
        //
        // Ok((remainder, metadata))
    }
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

