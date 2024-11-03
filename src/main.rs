mod string_parser;
mod parsers;
mod brew_command;

use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, multispace0, space0};
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

use string_parser::string;
use parsers::*;

#[derive(Debug)]
enum Command<'a> {
    Brew(Brew<'a>),
    Tap(Tap<'a>),
}

#[derive(Debug, Clone, Default)]
struct Tap<'a> {
    user_repo: &'a str,
    url: Option<&'a str>,
}

#[derive(Debug, Clone)]
struct Brew<'a> {
    command: &'a str,
    // args: Vec<&'a str>,
    params: HashMap<&'a str, Param<'a>>,
}

fn parse_param(input: &str) -> IResult<&str, (&str, Param)> {
    let (remainder, key) = terminated(alpha1, terminated(tag(":"), space0))(input)?;
    let (remainder, value) = match key {
        "args" => alt((parse_object, parse_list))(remainder),
        "link" => alt((
            value(Param::Boolean(true), tag("true")),
            value(Param::String("override"), tag(":override"))
        ))(remainder),
        unknown => panic!("Unknonw parameter {unknown}"),
    }?;

    Ok((remainder, (key, value)))
}

fn parse_params(input: &str) -> IResult<&str, HashMap<&str, Param>> {
    let (remainder, values) = separated_list0(
        terminated(tag(","), space0),
        parse_param
    )(input)?;

    let mut params: HashMap<&str, Param> = HashMap::new();
    for (key, value) in values {
        params.insert(key, value);
    };

    Ok((remainder, params))
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

fn parse_brew(input: &str) -> IResult<&str, Brew> {
    let (remainder, target) = string::<()>(input).unwrap();
    let (remainder, result) = is_last(remainder)?;

    if result {
        // Build the object we need to return
        let brew = Brew {
            command: target,
            // args: Vec::new()
            params: HashMap::new(),
        };

        return Ok((remainder, brew))
    }

    let (remainder, values) = parse_params(remainder)?;

    // Build the object we need to return
    let brew = Brew {
        command: target,
        params: values
    };

    Ok((remainder, brew))
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
            let (remainder, brew) = parse_brew(remainder)?;
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

