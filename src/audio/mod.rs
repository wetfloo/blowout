pub mod piece;

use std::{path::Path, time::Duration};

use anyhow;
use hound::{SampleFormat, WavSpec, WavWriter};

use self::piece::WriteAudio;
pub use self::piece::{Fade, Static};

pub struct TemporalPiece(pub Piece, pub Duration);

trait SampleCount {
    fn sample_count(&self, sample_rate: u32) -> u64;
}

impl SampleCount for Duration {
    fn sample_count(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.as_secs_f64();
        let unrounded = sample_rate as f64 * duration_secs;

        unrounded.round() as u64
    }
}

pub enum Piece {
    Static(Static),
    Fadeout(Fade),
}

pub fn make_audio<Temporals>(temporals: Temporals, spec: &AudioSpec) -> anyhow::Result<()>
where
    Temporals: IntoIterator<Item = TemporalPiece>,
{
    let mut writer = WavWriter::create(spec.file_path, spec.wav_spec)?;
    let sample_rate = spec.wav_spec.sample_rate;

    for temporal in temporals {
        let TemporalPiece(piece, duration) = temporal;
        let sample_count = duration.sample_count(sample_rate);

        match piece {
            Piece::Static(p) => p.write(&mut writer, sample_rate, sample_count)?,
            Piece::Fadeout(p) => p.write(&mut writer, sample_rate, sample_count)?,
        }
    }

    writer.finalize()?;

    Ok(())
}

pub struct AudioSpec<'a> {
    wav_spec: WavSpec,
    file_path: &'a Path,
}

impl<'a> AudioSpec<'a> {
    pub fn new(file_path: &'a Path) -> Self {
        let wav_spec = hound::WavSpec {
            bits_per_sample: 32,
            channels: 1,
            sample_rate: 48000,
            sample_format: SampleFormat::Float,
        };

        Self {
            wav_spec,
            file_path,
        }
    }
}
