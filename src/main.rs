use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use input::Args;

mod input;
mod regex;
mod speed;
mod unit;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let unit = args.unit.try_into()?;
    let _speeds: Vec<_> = reader
        .lines()
        .map(|line| speed::get_speed(&unit, line?.as_str()))
        .collect::<Result<_, _>>()?;

    Ok(())
}
