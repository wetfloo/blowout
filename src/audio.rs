use std::{path::Path, time::Duration};

use hound::SampleFormat;

use crate::speed::Speed;

pub fn make_audio(_speeds: &[Speed], spec: &AudioSpec) -> anyhow::Result<()> {
    let mut writer = hound::WavWriter::create(spec.file_path, spec.audio_spec)?;

    let seconds = spec.duration.as_secs_f64();
    let total_samples = spec.audio_spec.sample_rate as f64 * seconds;
    let iterations = total_samples.round() as u64;
    
    for _ in 0..iterations {
        let sample: f32 = 44000.0;
        writer.write_sample(sample)?;
    }

    Ok(())
}

pub struct AudioSpec<'a> {
    audio_spec: hound::WavSpec,
    duration: Duration,
    file_path: &'a Path,
}

impl<'a> AudioSpec<'a> {
    pub fn new(file_path: &'a Path, duration: Duration) -> Self {
        let audio_spec = hound::WavSpec {
            bits_per_sample: 32,
            channels: 1,
            sample_rate: 48000,
            sample_format: SampleFormat::Float,
        };

        Self {
            audio_spec,
            file_path,
            duration,
        }
    }
}
