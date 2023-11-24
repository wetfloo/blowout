use clap::Parser;
use input::{Args, InputError};
use unit::MeasurementUnit;

mod input;
mod regex;
mod unit;
mod speed;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let unit: MeasurementUnit = args
        .unit
        .ok_or_else(|| InputError::NoInput)
        .and_then(|s| s.try_into())?;
    let _speed = speed::get_speed(unit, todo!())?;

    Ok(())
}
