use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use audio::{AudioSpec, Piece, Static, TemporalPiece};
use clap::Parser;
use cli::Args;
use speed::Speed;

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
    let frequency_mulitplier = args.frequency_multiplier;
    let duration = Duration::from_millis(args.sample_duration_ms);

    let speeds: Vec<_> = reader
        .lines()
        .filter(|line_res| match line_res {
            Ok(line_res) => !line_res.trim().is_empty(),
            Err(_) => true,
        })
        .filter_map(|line_res| line_res.ok())
        .filter_map(|line| speed::get_speed(line, &unit).ok())
        .map(|Speed(val)| val * frequency_mulitplier)
        .map(|freq| {
            Piece::Static(Static {
                frequency: freq,
                amplitude: args.amplitude,
            })
        })
        .map(|piece| TemporalPiece(piece, duration))
        .collect();

    if speeds.is_empty() {
        return Err(NoValues.into());
    }

    let audio_spec = AudioSpec::new(&Path::new(&args.file_name));
    audio::make_audio(speeds.into_iter(), &audio_spec)?;

    Ok(())
}
