use std::{
    fs::File,
    io::{BufRead, BufReader}, path::Path,
};

use clap::Parser;
use input::{Args, InputError};

use unit::MeasurementUnit;

mod input;
mod regex;
mod speed;
mod unit;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let unit: MeasurementUnit = args
        .unit
        .ok_or_else(|| InputError::NoInput)
        .and_then(|s| s.try_into())?;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let _speeds: Vec<_> = reader
        .lines()
        .map(|line| speed::get_speed(&unit, line?.as_str()))
        .collect::<Result<_, _>>()?;

    Ok(())
}
