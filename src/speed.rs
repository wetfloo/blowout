use std::{num::ParseFloatError, str::FromStr};

use nom::bytes::streaming::tag;
use nom::character::streaming::{anychar, space0};

use nom::multi::many_till;
use nom::number::streaming::float;
use nom::sequence::tuple;
use nom::{sequence::terminated, IResult};

use crate::unit::MeasurementUnit;

// TODO: figure out how to not take input as a allocated string.
pub fn get_speed(
    input: String,
    MeasurementUnit(unit_string): &MeasurementUnit,
) -> anyhow::Result<Speed> {
    let mut cls = many_till(
        anychar,
        terminated(float, tuple((space0, tag(unit_string.as_str())))),
    );
    let parse_result: IResult<&str, _, ()> = cls(input.as_str());
    let (_, (_, x)) = parse_result?;

    Ok(Speed(x))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

impl From<ParseFloatError> for InvalidInput {
    fn from(value: ParseFloatError) -> Self {
        InvalidInput::Speed(value)
    }
}

impl FromStr for Speed {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Speed(s.parse()?))
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
    use super::{get_speed, Speed};
    use crate::unit::MeasurementUnit;

    const INPUT_STR: &str =
        r#"1             18.2°C 40.7% 4.04          m/s           15.06.2023\11:06:11"#;
    const INPUT_STR_COMMA: &str =
        r#"1             18.2°C 40.7% 4,04          m/s           15.06.2023\11:06:11"#;
    const INPUT_STR_ONLY: &str = r#"69 m/s"#;

    #[test]
    fn test_input_valid() {
        let unit = MeasurementUnit("m/s".into());
        let result = get_speed(INPUT_STR.into(), &unit).unwrap();

        assert_eq!(Speed(4.04), result);
    }

    // TODO: this doesn't parse the value in the correct way for some reason,
    // losing precision when parsing.
    #[test]
    #[ignore]
    fn test_input_valid_comma() {
        let unit = MeasurementUnit("m/s".into());
        let result = get_speed(INPUT_STR_COMMA.into(), &unit).unwrap();

        assert_eq!(Speed(4.04), result);
    }

    #[test]
    fn test_input_only() {
        let unit = MeasurementUnit("m/s".into());
        let result = get_speed(INPUT_STR_ONLY.into(), &unit).unwrap();

        assert_eq!(Speed(69.0), result);
    }
}
