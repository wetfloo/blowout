use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use clap::Parser;
use cli::Args;

mod audio;
mod cli;
mod regex;
mod speed;
mod unit;

#[derive(Debug, thiserror::Error)]
#[error("no valid values found in the file")]
struct NoValues;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.filepath)?;
    let reader = BufReader::new(file);

    let unit = args.unit.try_into()?;
    let speeds: Vec<_> = reader
        .lines()
        .filter(|line_res| match line_res {
            Ok(line_res) => !line_res.trim().is_empty(),
            Err(_) => true,
        })
        .map(|line_res| speed::get_speed(&unit, &line_res?))
        .filter_map(|speed_res| speed_res.ok())
        .collect();
    if speeds.is_empty() {
        return Err(NoValues.into());
    }

    let audio_spec = audio::AudioSpec::new(
        &Path::new("output.wav"),
        Duration::from_secs(args.duration_seconds),
    );
    audio::make_audio(&speeds, &audio_spec)?;

    Ok(())
}
