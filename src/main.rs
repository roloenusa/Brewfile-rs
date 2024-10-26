use std::error::Error;

use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::IResult;

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
   many0(tag("abc"))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(parse("abcabc"), Ok(("", vec!["abc", "abc"])));
    assert_eq!(parse("abc123"), Ok(("123", vec!["abc"])));
    assert_eq!(parse("123123"), Ok(("123123", vec![])));

    Ok(())
}

