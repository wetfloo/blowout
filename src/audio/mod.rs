pub mod piece;

use std::{path::Path, time::Duration};

use anyhow;
use hound::{SampleFormat, WavSpec, WavWriter};

use self::piece::WriteAudio;
pub use self::piece::{Fade, Static};

#[derive(Clone, Debug)]
pub struct TemporalPiece(pub Piece, pub Duration);

trait SampleCount {
    fn sample_count(&self, sample_rate: u32) -> u64;
}

impl SampleCount for Duration {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn sample_count(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.as_secs_f64();
        let unrounded = f64::from(sample_rate) * duration_secs;

        assert!(unrounded > 0.0);
        // Should never be negative here.
        unrounded.round() as u64
    }
}

#[derive(Clone, Debug)]
pub enum Piece {
    Static(Static),
    Fadeout(Fade),
}

pub fn make<Temporals>(temporals: Temporals, spec: &Spec) -> anyhow::Result<()>
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

pub struct Spec<'a> {
    wav_spec: WavSpec,
    file_path: &'a Path,
}

impl<'a> Spec<'a> {
    pub const fn new(file_path: &'a Path) -> Self {
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
