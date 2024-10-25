use std::error::Error;

use nom::bytes::complete::{take_until, take_while};
use nom::sequence::terminated;
use nom::IResult;

fn parse_sentence(input: &str) -> IResult<&str, &str> {
    terminated(take_until("."), take_while(|c| c == '.' || c == ' '))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining, parsed) = parse_sentence("I am Tom. I write Rust")?;
    assert_eq!(parsed, "I am Tom");
    assert_eq!(remaining, "I write Rust");

    let parser_error = parse_sentence("Not a sentence (no period at the end)");
    assert!(parser_error.is_err());

    Ok(())
}

