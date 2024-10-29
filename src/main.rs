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
}

#[derive(Debug, Clone)]
struct Brew<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

fn parse_list(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        terminated(tag("["), space0),
        separated_list0(tuple((space0, tag(","), space0)), string),
        preceded(space0, tag("]")),
    )(input)
}

fn parse_brew(input: &str) -> IResult<&str, Brew> {
    let (remainder, target) = string::<()>(input).unwrap();
    let (remainder, _) = tuple((space0, tag(","), space0))(remainder)?;

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
    let (remainder, _brew_command) = terminated(tag("brew"), space0)(input)?;

    match _brew_command {
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

