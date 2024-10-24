use std::error::Error;

use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn parse_abc_or_def(input: &str) -> IResult<&str, &str> {
    alt((
        tag("abc"),
        tag("def")
    ))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_input("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    assert!(parse_input("defWorld").is_err());


    // Alt combinator
    let (leftover_input, output) = parse_abc_or_def("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    let (leftover_input, output) = parse_abc_or_def("defWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "def");

    assert!(parse_abc_or_def("ghiWorld").is_err());

    Ok(())
}

