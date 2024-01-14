use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use audio::{Piece, Static, TemporalPiece};
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
    let speeds: Vec<_> = reader
        .lines()
        .filter(|line_res| match line_res {
            Ok(line_res) => !line_res.trim().is_empty(),
            Err(_) => true,
        })
        .filter_map(|line_res| line_res.ok().and_then(|v| speed::get_speed(v, &unit).ok()))
        .map(|Speed(speed)| speed)
        .collect();
    if speeds.is_empty() {
        return Err(NoValues.into());
    }
    let frequency_mulitplier = args.frequency_multiplier;
    let duration = Duration::from_millis(args.sample_duration_ms);
    let sounds: Vec<_> = speeds
        .iter()
        .map(|speed| speed * frequency_mulitplier)
        .map(|freq| {
            Piece::Static(Static {
                frequency: freq,
                amplitude: 0.9,
            })
        })
        .map(|piece| TemporalPiece(piece, duration))
        .collect();

    let audio_spec = audio::AudioSpec::new(&Path::new("output.wav"));
    let iter = sounds.into_iter();
    audio::make_audio(iter, &audio_spec)?;

    Ok(())
}

trait LinesLossy {
    fn lines_lossy() -> anyhow::Result<()>;
}
