use nom::branch::alt;
use nom::character::complete::{multispace0, space0};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;
use string_parser::string;

use crate::string_parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Param<'a> {
    String(&'a str),
    Boolean(bool),
    Map(Vec<(&'a str, &'a str)>),
    List(Vec<&'a str>),
}

pub fn is_last(input: &str) -> IResult<&str, bool> {
    let (remainder, comma) = alt((parse_spacer, multispace0))(input)?;
    match comma {
        "," => Ok((remainder, false)),
        _ => Ok((remainder, true)),
    }
}

pub fn parse_spacer(input: &str) -> IResult<&str, &str> {
    preceded(space0, terminated(tag(","), space0))(input)
}

pub fn parse_list(input: &str) -> IResult<&str, Param> {
    let (remainder, list) = delimited(
        terminated(tag("["), space0),
        separated_list0(parse_spacer, string),
        preceded(space0, tag("]")),
    )(input)?;

    Ok((remainder, Param::List(list)))
}

pub fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        string,
        terminated(tag(":"), space0),
        string
    )(input)
}

pub fn parse_object(input: &str) -> IResult<&str, Param> {
    let (remainder, pairs) = delimited(
        terminated(tag("{"), space0),
        separated_list0(parse_spacer, key_value),
        preceded(space0, tag("}")),
    )(input)?;
    Ok((remainder, Param::Map(pairs)))
}

