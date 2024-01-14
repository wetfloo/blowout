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

#[derive(Debug, Clone, Copy)]
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
