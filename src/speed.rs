use std::{num::ParseFloatError, str::FromStr};

use nom::bytes::streaming::tag;
use nom::character::streaming::{anychar, space0};

use nom::multi::many_till;
use nom::number::streaming::float;
use nom::sequence::tuple;
use nom::{sequence::terminated, IResult};

pub fn get<'a>(input: &'a str, measurement_unit: &'a str) -> IResult<&'a str, Speed> {
    let mut cls = many_till(
        anychar,
        terminated(float, tuple((space0, tag(measurement_unit)))),
    );
    let parse_result: IResult<&str, _> = cls(input);
    let (_, (_, x)) = parse_result?;

    Ok((input, Speed(x)))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

impl From<ParseFloatError> for InvalidInput {
    fn from(value: ParseFloatError) -> Self {
        Self::Speed(value)
    }
}

impl FromStr for Speed {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidInput {
    #[error("invalid float for speed")]
    Speed(ParseFloatError),
    #[error("speed value is not provided")]
    NoSpeed,
    #[error("malformed input, couldn't parse anything")]
    Malformed,
}

#[cfg(test)]
mod tests {
    use super::{get, Speed};

    const INPUT_STR: &str =
        r"1             18.2°C 40.7% 4.04          m/s           15.06.2023\11:06:11";
    const INPUT_STR_COMMA: &str =
        r"1             18.2°C 40.7% 4,04          m/s           15.06.2023\11:06:11";
    const INPUT_STR_ONLY: &str = r"69 m/s";

    #[test]
    fn test_input_valid() {
        let unit = "m/s";
        let result = get(INPUT_STR, unit);

        assert_eq!(Ok(Speed(4.04)), result.map(|(_, val)| val));
    }

    // TODO: this doesn't parse the value in the correct way for some reason,
    // losing precision when parsing.
    #[test]
    #[ignore]
    fn test_input_valid_comma() {
        let unit = "m/s";
        let result = get(INPUT_STR, unit);

        assert_eq!(Ok(Speed(4.04)), result.map(|(_, val)| val));
    }

    #[test]
    fn test_input_only() {
        let unit = "m/s";
        let result = get(INPUT_STR_ONLY, unit);

        assert_eq!(Ok(Speed(69.0)), result.map(|(_, val)| val));
    }
}
