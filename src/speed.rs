use std::{num::ParseFloatError, str::FromStr};

use crate::{regex::get_regex, unit::MeasurementUnit};

pub fn get_speed(unit: MeasurementUnit, input: &str) -> anyhow::Result<Speed> {
    let regex = get_regex(unit)?;
    let capture = regex
        .captures(input)
        .ok_or_else(|| InvalidInput::Malformed)?
        .iter()
        .skip(1)
        .find_map(|mtch| mtch.map(|m| m.as_str()))
        .ok_or_else(|| InvalidInput::NoSpeed)?;

    Ok(capture.parse()?)
}

#[derive(Debug, Clone, Copy)]
pub struct Speed(pub f64);

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

