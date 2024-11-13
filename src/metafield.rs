use nom::{bytes::complete::{tag, take_until}, character::complete::alpha1, sequence::preceded, IResult};

#[derive(Debug, Clone)]
pub enum Metafield<'a> {
    Description(&'a str),
    Optional
}

impl<'a> Metafield<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {

        // Check if this is the last parameter on the set
        let (remainder, key) = preceded(tag("@"), alpha1)(input)?;
        match key {
            "description" => {
                let (remainder, value) = take_until("\n")(remainder)?;
                Ok((remainder, Metafield::Description(value.trim())))
            },
            "optional" => {
                let (remainder, _) = take_until("\n")(remainder)?;
                Ok((remainder, Metafield::Optional))
            },
            unknown => panic!("Unknown parameter {unknown}"),
        }
    }
}

