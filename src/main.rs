use std::error::Error;

use nom::bytes::complete::tag;
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_input("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    assert!(parse_input("defWorld").is_err());
    Ok(())
}

