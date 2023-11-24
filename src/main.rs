use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use cli::Args;

mod cli;
mod regex;
mod speed;
mod unit;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.filepath)?;
    let reader = BufReader::new(file);

    let unit = args.unit.try_into()?;
    let _speeds: Vec<_> = reader
        .lines()
        .map(|line_res| speed::get_speed(&unit, &line_res?))
        .filter_map(|speed_res| speed_res.ok())
        .collect();

    Ok(())
}
