use std::error::Error;

use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::value;
use nom::IResult;

fn parse_bool(input: &str) -> IResult<&str, bool> {
    // either, parse `"true"` -> `true`; or `"false"` -> `false`, or error.
    alt((
        value(true, tag("true")),
        value(false, tag("false")),
    ))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parses the `"true"` out.
    let (remaining, parsed) = parse_bool("true|false")?;
    assert_eq!(parsed, true);
    assert_eq!(remaining, "|false");

    // if we forget about the "|" we get an error.
    let parsing_error = parse_bool(remaining);
    assert!(parsing_error.is_err());

    // Skipping the first byte gives us `false`!
    let (remaining, parsed) = parse_bool(&remaining[1..])?;
    assert_eq!(parsed, false);
    assert_eq!(remaining, "");
    Ok(())
}

