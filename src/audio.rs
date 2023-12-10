#![allow(dead_code, unused)]
use std::{path::Path, time::Duration};

use hound::{SampleFormat, WavSpec, WavWriter};

pub fn make_audio(pieces: &[AudioPiece], spec: &AudioSpec) -> anyhow::Result<()> {
    let sample_rate = spec.audio_spec.sample_rate;
    let mut writer = WavWriter::create(spec.file_path, spec.audio_spec)?;

    for piece in pieces {
        let samples_count = piece.duration_samples(sample_rate);
        let every_nth_sample = piece.every_nth_sample(sample_rate) as u64;
        for sample_index in 0..samples_count {
            let amplitude = if sample_index % every_nth_sample == 0 {
                piece.amplitude
            } else {
                0f32
            };
            writer.write_sample(amplitude)?;
        }
    }

    Ok(())
}

pub struct AudioPiece {
    pub frequency: u16,
    pub amplitude: f32,
    pub duration: Duration,
}

impl AudioPiece {
    fn every_nth_sample(&self, sample_rate: u32) -> u32 {
        sample_rate / self.frequency as u32
    }

    fn duration_samples(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.duration.as_secs_f64();
        let unrounded = sample_rate as f64 * duration_secs;

        unrounded.round() as u64
    }
}

pub struct AudioSpec<'a> {
    audio_spec: WavSpec,
    file_path: &'a Path,
}

impl<'a> AudioSpec<'a> {
    pub fn new(file_path: &'a Path) -> Self {
        let audio_spec = hound::WavSpec {
            bits_per_sample: 32,
            channels: 1,
            sample_rate: 48000,
            sample_format: SampleFormat::Float,
        };

        Self {
            audio_spec,
            file_path,
        }
    }
}
