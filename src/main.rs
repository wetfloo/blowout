use clap::Parser;
use input::{Args, InputError};
use regex::get_regex;
use unit::MeasurementUnit;

mod input;
mod regex;
mod unit;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let unit: MeasurementUnit = args
        .unit
        .ok_or_else(|| InputError::NoInput)
        .and_then(|s| s.try_into())?;

    let _regex = get_regex(unit)?;

    Ok(())
}
