use std::{f32::consts::PI, path::Path, time::Duration};

use hound::{SampleFormat, WavSpec, WavWriter};

pub fn make_audio<Pieces>(pieces: Pieces, spec: &AudioSpec) -> anyhow::Result<()>
where
    Pieces: IntoIterator<Item = AudioPiece>,
{
    let mut writer = WavWriter::create(spec.file_path, spec.wav_spec)?;
    let sample_rate = spec.wav_spec.sample_rate;

    for piece in pieces {
        let duration_samples = piece.duration_samples(sample_rate);

        // This will start clicking with smaller duration windows,
        // I guess it happens because it cuts the wave too early?
        let coefficient_iter = (0..duration_samples).map(|x| x as f32 / sample_rate as f32);
        for coefficient in coefficient_iter {
            let base_value = (2.0 * PI * coefficient * piece.frequency as f32).cos();
            let sample = base_value * piece.amplitude;

            writer.write_sample(sample)?;
        }
    }
    writer.finalize()?;

    Ok(())
}

/// Represents a small portion of an audio file.
/// * `frequency` - is specified in hertz
/// * `amplitude` - defines how loud this piece is. Values higher than `1.0` lead to distortion
/// * `duration` - The amount of time the piece will be playing for
pub struct AudioPiece {
    pub frequency: u16,
    pub amplitude: f32,
    pub duration: Duration,
}

impl AudioPiece {
    fn duration_samples(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.duration.as_secs_f64();
        let unrounded = sample_rate as f64 * duration_secs;

        unrounded.round() as u64
    }
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
