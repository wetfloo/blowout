use std::f32::consts::PI;
use std::io::{Seek, Write};
use std::path::Path;
use std::time;

use anyhow;
use hound::{SampleFormat, WavSpec, WavWriter};

pub enum Piece {
    Static(Static),
    Fadeout(Fade),
}

pub fn make_audio<Pieces>(pieces: Pieces, spec: &AudioSpec) -> anyhow::Result<()>
where
    Pieces: IntoIterator<Item = Piece>,
{
    let mut writer = WavWriter::create(spec.file_path, spec.wav_spec)?;
    let sample_rate = spec.wav_spec.sample_rate;

    for piece in pieces {
        match piece {
            Piece::Static(p) => p.write(&mut writer, sample_rate)?,
            Piece::Fadeout(p) => p.write(&mut writer, sample_rate)?,
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

trait SampleCount {
    fn sample_count(&self, sample_rate: u32) -> u64;
}

trait DurationSecsF64 {
    fn duration(&self) -> f64;
}

impl<T: DurationSecsF64> SampleCount for T {
    fn sample_count(&self, sample_rate: u32) -> u64 {
        let duration_secs = self.duration();
        let unrounded = sample_rate as f64 * duration_secs;

        unrounded.round() as u64
    }
}

trait WriteAudio {
    fn write<W>(&self, writer: &mut WavWriter<W>, sample_rate: u32) -> anyhow::Result<()>
    where
        W: Seek + Write;
}

/// Represents a small portion of an audio file that doesn't really change in amplitude during its
/// lifetime.
/// * `frequency` - is specified in hertz
/// * `amplitude` - defines how loud this piece is. Values higher than `1.0` lead to distortion
/// * `duration` - The amount of time the piece will be playing for
pub struct Static {
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: time::Duration,
}

impl WriteAudio for Static {
    fn write<W>(&self, writer: &mut WavWriter<W>, sample_rate: u32) -> anyhow::Result<()>
    where
        W: Seek + Write,
    {
        let sample_count = self.sample_count(sample_rate);

        let coefficient_iter = (0..sample_count).map(|x| x as f32 / sample_rate as f32);
        for coefficient in coefficient_iter {
            let base_value = (2.0 * PI * coefficient * self.frequency).cos();
            let sample = base_value * self.amplitude;

            writer.write_sample(sample)?;
        }

        Ok(())
    }
}

impl DurationSecsF64 for Static {
    fn duration(&self) -> f64 {
        self.duration.as_secs_f64()
    }
}

#[derive(Default)]
pub struct Fade {
    pub duration: time::Duration,
    pub end_amplitude: f32,
    pub frequency: f32,
    pub reverse: bool,
}

impl WriteAudio for Fade {
    fn write<W>(&self, writer: &mut WavWriter<W>, sample_rate: u32) -> anyhow::Result<()>
    where
        W: Seek + Write,
    {
        let sample_count = self.sample_count(sample_rate);
        let end_amplitude = self.end_amplitude;
        let fraction = end_amplitude / sample_count as f32;

        let sample_iter = (0..sample_count)
            .map(|x| x as f32 * fraction)
            .map(|part| {
                if self.reverse {
                    part
                } else {
                    end_amplitude - part
                }
            })
            .enumerate()
            .map(|(index, amplitude)| {
                let coefficient = index as f32 / sample_rate as f32;
                let base_value = (2.0 * PI * coefficient * self.frequency).cos();
                base_value * amplitude
            });

        for sample in sample_iter {
            writer.write_sample(sample)?;
        }

        Ok(())
    }
}

impl DurationSecsF64 for Fade {
    fn duration(&self) -> f64 {
        self.duration.as_secs_f64()
    }
}
