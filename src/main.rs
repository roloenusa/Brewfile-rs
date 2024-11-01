mod string_parser;

use nom::branch::alt;
use nom::bytes::complete::{escaped, escaped_transform, is_not, take_while_m_n};
use nom::character::complete::{alpha1, alphanumeric1, line_ending, multispace0, one_of, space0};
use nom::character::streaming::{char, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::{cut, map, map_opt, map_res, peek, rest, value, verify};
use nom::complete::take;
use nom::error::{context, ContextError, FromExternalError, ParseError};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::{IResult, Parser};
use string_parser::string;

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
    args: Vec<&'a str>,
}

fn is_last(input: &str) -> IResult<&str, bool> {
    let (remainder, comma) = alt((parse_spacer, multispace0))(input)?;
    match comma {
        "," => Ok((remainder, false)),
        _ => Ok((remainder, true)),
    }
}

fn parse_spacer(input: &str) -> IResult<&str, &str> {
    preceded(space0, terminated(tag(","), space0))(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        terminated(tag("["), space0),
        separated_list0(parse_spacer, string),
        preceded(space0, tag("]")),
    )(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        string,
        terminated(tag(":"), space0),
        string
    )(input)
}

fn parse_object(input: &str) -> IResult<&str, Vec<&str>> {
    let (remainder, result) = delimited(
        terminated(tag("{"), space0),
        separated_list0(parse_spacer, key_value),
        preceded(space0, tag("}")),
    )(input)?;

    let mut r: Vec<&str> = Vec::new();
    for (a, b) in result {
        r.push(a);
        r.push(b);
    }

    Ok((remainder, r))
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
            args: Vec::new()
        };

        return Ok((remainder, brew))
    }

    // Grab the parameter, ignore white spaces after.
    let (remainder, _param) = terminated(tag("args"), tag(":"))(remainder)?;
    let (remainder, _) = space0(remainder)?;

    // Parse the list of arguments
    let (remainder, list) = alt((
        parse_object,
        parse_list,
    ))(remainder)?;

    // Build the object we need to return
    let brew = Brew {
        command: target,
        args: list,
    };

    Ok((remainder, brew))
}

fn parse_command(input: &str) -> IResult<&str, Command>{
    // Commands should always be followed by a space
    let (remainder, _brew_command) = terminated(
        alphanumeric1,
        space0)
        .parse(input)?;

    match _brew_command {
        "tap" => {
            let (remainder, tap) = parse_tap(remainder)?;
            Ok((remainder, Command::Tap(tap)))
        }
        "brew" => {
            let (remainder, brew) = parse_brew(remainder)?;
            Ok((remainder, Command::Brew(brew)))
        }
        unknown => panic!("Unknown command: {unknown}"),
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

