use std::{
    f32::consts::PI,
    io::{Seek, Write},
    path::Path,
    time::Duration,
};

use hound::{SampleFormat, WavSpec, WavWriter};

pub fn make_audio<Pieces>(pieces: Pieces, spec: &AudioSpec) -> anyhow::Result<()>
where
    Pieces: IntoIterator<Item = AudioPiece>,
{
    let mut writer = WavWriter::create(spec.file_path, spec.wav_spec)?;
    let sample_rate = spec.wav_spec.sample_rate;

    for piece in pieces {
        let writeable_static = piece.writeable(sample_rate);
        write_sample(&mut writer, &writeable_static, sample_rate)?;

        let writeables = piece.fadeout_writeable(sample_rate);
        for writeable in writeables {
            write_sample(&mut writer, &writeable, sample_rate)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

fn write_sample<W>(
    writer: &mut WavWriter<W>,
    writeable: &Writeable,
    sample_rate: u32,
) -> anyhow::Result<()>
where
    W: Seek + Write,
{
    let coefficient_iter = (0..writeable.samples_count).map(|x| x as f32 / sample_rate as f32);
    for coefficient in coefficient_iter {
        let base_value = (2.0 * PI * coefficient * writeable.frequency).cos();
        let sample = base_value * writeable.amplitude;

        writer.write_sample(sample)?;
    }

    Ok(())
}

struct Writeable {
    samples_count: u64,
    frequency: f32,
    amplitude: f32,
}

trait DurationSamples {
    fn duration_samples(&self, sample_rate: u32) -> u64;
}

/// Represents a small portion of an audio file.
/// * `frequency` - is specified in hertz
/// * `amplitude` - defines how loud this piece is. Values higher than `1.0` lead to distortion
/// * `duration` - The amount of time the piece will be playing for
pub struct AudioPiece {
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: Duration,
    pub fadeout: Fadeout,
}

impl AudioPiece {
    /// The returned [Writeable] is always the same, no need to repeat it over and over in a [Vec].
    fn writeable(&self, sample_rate: u32) -> Writeable {
        Writeable {
            samples_count: self.duration_samples(sample_rate),
            amplitude: self.amplitude,
            frequency: self.frequency,
        }
    }

    /// This uses linear interpolation to fade out the wave, returning a [Vec] of [Writeable]s with gradually decreasing amplitude.
    fn fadeout_writeable(&self, sample_rate: u32) -> Vec<Writeable> {
        let samples_count = self.fadeout.duration_samples(sample_rate);
        let end_amplitude = self.fadeout.end_amplitude;
        let subtractable = end_amplitude / samples_count as f32;

        (0..samples_count)
            .map(|mul| end_amplitude - subtractable * mul as f32)
            .map(|amplitude| Writeable {
                samples_count: self.fadeout.duration_samples(sample_rate),
                frequency: self.frequency,
                amplitude,
            })
            .collect()
    }
}

impl DurationSamples for AudioPiece {
    fn duration_samples(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.duration.as_secs_f64();
        let unrounded = sample_rate as f64 * duration_secs;

        unrounded.round() as u64
    }
}

#[derive(Default)]
pub struct Fadeout {
    duration: Duration,
    end_amplitude: f32,
}

impl DurationSamples for Fadeout {
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
