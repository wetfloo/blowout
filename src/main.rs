use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use audio::{Piece, Spec, Static, TemporalPiece};
use clap::Parser;
use cli::Args;
use speed::Speed;

mod audio;
mod cli;
mod speed;

#[derive(Debug, thiserror::Error)]
#[error("no valid values found in the file")]
struct NoValues;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.filepath)?;
    let reader = BufReader::new(file);

    let duration = Duration::from_millis(args.sample_duration_ms);

    let speeds: Vec<_> = reader
        .lines()
        .filter(|line_res| match line_res {
            Ok(line_res) => !line_res.trim().is_empty(),
            Err(_) => true,
        })
        .filter_map(Result::ok)
        .map(|line| {
            if line.chars().any(|c| c == ',') {
                line.replace(',', ".")
            } else {
                line
            }
        })
        .filter_map(|line| {
            let speed_res = speed::get(&line, &args.measurement_unit);
            match speed_res {
                Ok((_, Speed(val))) => Some(val),
                Err(_) => None,
            }
        })
        .map(|freq| {
            Piece::Static(Static {
                frequency: freq.mul_add(args.frequency_multiplier, args.frequency_term),
                amplitude: args.amplitude,
            })
        })
        .map(|piece| TemporalPiece(piece, duration))
        .collect();

    if speeds.is_empty() {
        return Err(NoValues.into());
    }

    let audio_spec = Spec::new(&Path::new(&args.file_name));
    audio::make(speeds.into_iter(), &audio_spec)?;

    Ok(())
}
