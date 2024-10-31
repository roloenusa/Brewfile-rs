mod string_parser;

use nom::branch::alt;
use nom::bytes::complete::{escaped, escaped_transform, is_not, take_while_m_n};
use nom::character::complete::{alphanumeric1, line_ending, multispace0, one_of, space0};
use nom::character::streaming::{char, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::{cut, map, map_opt, map_res, rest, value, verify};
use nom::error::{context, ContextError, FromExternalError, ParseError};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{delimited, preceded, terminated, tuple};
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
    let (remainder, comma) = preceded(space0, terminated(tag(","), space0))(input)?;
    Ok((remainder, comma))
}

fn parse_list(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        terminated(tag("["), space0),
        separated_list0(parse_spacer, string),
        preceded(space0, tag("]")),
    )(input)
}

fn parse_tap(input: &str) -> IResult<&str, Tap> {
    let (remainder, user_repo) = string::<()>(input).unwrap();
    // let mut tap = Tap ser_repo);

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
    let (remainder, list) = parse_list(remainder)?;

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
        alt((tag("brew"), tag("tap"))),
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
        _ => panic!("stuff"),
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

    let result = parse_input(&src).unwrap();
    println!("{:#?}", result);
}

