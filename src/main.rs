
use nom::branch::alt;
use nom::bytes::complete::{escaped, escaped_transform, is_not, take_while_m_n};
use nom::character::complete::{alphanumeric1, line_ending, multispace0, one_of, space0};
use nom::character::streaming::{char, multispace1};
use nom::bytes::complete::tag;
use nom::combinator::{cut, map, map_opt, map_res, rest, value, verify};
use nom::error::{context, ContextError, FromExternalError, ParseError};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::{IResult, Parser};

#[derive(Debug)]
enum Command {
    Brew(String),
}

/// A nom parser has the following signature:
/// `Input -> IResult<Input, Output, Error>`, with `IResult` defined as:
/// `type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;`
///
/// most of the times you can ignore the error type and use the default (but this
/// examples shows custom error types later on!)
///
/// Here we use `&str` as input type, but nom parsers can be generic over
/// the input type, and work directly with `&[u8]` or any other type that
/// implements the required traits.
///
/// Finally, we can see here that the input and output type are both `&str`
/// with the same lifetime tag. This means that the produced value is a subslice
/// of the input data. and there is no allocation needed. This is the main idea
/// behind nom's performance.
fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    // escaped(alt((alphanumeric1, space0)), '\\', one_of("\"n\\"))(i)
    escaped(is_not("\\\""), '\\', one_of("\"nt\\"))(i)
}

/// this parser combines the previous `parse_str` parser, that recognizes the
/// interior of a string, with a parse to recognize the double quote character,
/// before the string (using `preceded`) and after the string (using `terminated`).
///
/// `context` and `cut` are related to error management:
/// - `cut` transforms an `Err::Error(e)` in `Err::Failure(e)`, signaling to
/// combinators like  `alt` that they should not try other parsers. We were in the
/// right branch (since we found the `"` character) but encountered an error when
/// parsing the string
/// - `context` lets you add a static string to provide more information in the
/// error chain (to indicate which parser had an error)
fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, &'a str, E> {
  context(
    "string",
    preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
  )
  .parse(input)
}

fn parse_command(input: &str) -> IResult<&str, Command>{
    let (remainder, _brew_command) = tag("brew")(input)?;

    match preceded(space0, string::<()>)(remainder) {
        Ok((remainder, target)) =>  {
            Ok((remainder, Command::Brew(target.to_string())))
        },
        Err(e) => panic!("Error parsing {:#?}", e),
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

    let (_remainder, result) = parse_input(&src).unwrap();
    println!("{:#?}", result);

    // match &result[0] {
    //     Command::Brew(text) => println!("---- text: {}", text),
    //     _ => panic!("wtf"),
    // }
    // println!("Result: {}", result[0]);
    // println!("Remainder: {}", remainder);
}

