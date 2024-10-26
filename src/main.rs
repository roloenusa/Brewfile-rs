use nom::{bytes::complete::tag, character::complete::multispace0, sequence::delimited, IResult};

fn parse(input: &str) -> IResult<&str, &str> {
    let (remainder, result) = delimited(
        multispace0,
        tag("brew"),
        multispace0
    )(input)?;

    Ok((remainder, result))
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{}", src);

    let (remainder, result) = parse(&src).unwrap();
    println!("Result: {result} - {remainder}");
}

