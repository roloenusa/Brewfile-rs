use std::collections::HashMap;

use nom::{branch::alt, character::complete::alpha1, combinator::value, bytes::complete::tag, sequence::terminated, IResult, Parser};
use nom::character::complete::space0;

use crate::string_parser::string;
use crate::{is_last, parse_list, parse_object, Param};


#[derive(Debug, Clone)]
pub struct BrewCommand<'a> {
    target: String,
    args: Param<'a>,
    link: LinkOptions,
}

#[derive(Debug, Clone)]
pub enum LinkOptions {
    None,
    On,
    Override,
}

impl<'a> BrewCommand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        // Allocate the structure data
        let mut brew = Self {
            target: String::new(),
            args: Param::String(""),
            link: LinkOptions::None,
        };

        // Get the initial command
        let (remainder, target) = string::<()>(input).unwrap();
        brew.target = target.to_string();

        // Check if this is the last parameter on the set
        let (remainder, last) = is_last(remainder)?;
        let mut last = last;
        let mut result_remainder = remainder;

        // Loop over all the parameters and update as needed
        while !last {
            let (remainder, key) = terminated(alpha1, terminated(tag(":"), space0))(result_remainder)?;
            let remainder = match key {
                "args" => {
                    let (remainder, value) = alt((parse_object, parse_list))(remainder)?;
                    brew.args = value;
                    remainder
                },
                "link" => {
                    let (remainder, value) = alt((
                        value(LinkOptions::On, tag("true")),
                        value(LinkOptions::Override, tag(":override"))
                    ))(remainder)?;
                    brew.link = value;
                    remainder
                },
                unknown => panic!("Unknonw parameter {unknown}"),
            };

            let (remainder, check) = is_last(remainder)?;
            last = check;
            result_remainder  = remainder;
        };

        Ok((result_remainder, brew))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_line() {
        let (remainder, brew) = BrewCommand::parse("\"target\" \n").unwrap();
        assert_eq!(brew.target, "target");
        assert_eq!(remainder, "");

        // Remainder must be after the line break
        let (remainder, brew) = BrewCommand::parse("\"target\" \nextra").unwrap();
        assert_eq!(brew.target, "target");
        assert_eq!(remainder, "extra");
    }

    #[test]
    fn parse_args() {
        let (remainder, brew) = BrewCommand::parse("\"target\", args: [\"hello\", \"world\"]\n").unwrap();
        match brew.args {
            Param::List(list) => assert_eq!(list, vec!["hello", "world"]),
            _ => assert!(false, "expected a list of params"),
        };
        assert_eq!(remainder, "");

        let (remainder, brew) = BrewCommand::parse("\"target\", args: {\"hello\": \"world\"}\n").unwrap();
        match brew.args {
            Param::Map(list) => assert_eq!(list, vec![("hello", "world")]),
            _ => assert!(false, "expected a map of params"),
        };
        assert_eq!(remainder, "");
    }

    #[test]
    fn parse_link() {
        let (remainder, brew) = BrewCommand::parse("\"target\", link: true \n").unwrap();
        assert_eq!(brew.target, "target");
        match brew.link {
            LinkOptions::On => assert!(true),
            _ => assert!(false, "expected LinkOptions::On"),
        };
        assert_eq!(remainder, "");

        let (remainder, brew) = BrewCommand::parse("\"target\", link: :override").unwrap();
        assert_eq!(brew.target, "target");
        match brew.link {
            LinkOptions::Override => assert!(true),
            _ => assert!(false, "expected LinkOptions::On"),
        };
        assert_eq!(remainder, "");
    }

}

