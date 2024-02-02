use std::f64::consts::PI;
use std::io::{Seek, Write};
use std::time;

use anyhow;
use hound::WavWriter;

pub(super) trait Writeable: Seek + Write {}

impl<T> Writeable for T where T: Seek + Write {}

pub(super) trait WriteAudio {
    fn write<W: Writeable>(
        &self,
        writer: &mut WavWriter<W>,
        sample_rate: u32,
        sample_count: u64,
    ) -> anyhow::Result<()>;
}

/// Represents a small portion of an audio file that doesn't really change in amplitude during its
/// lifetime.
/// * `frequency` - is specified in hertz
/// * `amplitude` - defines how loud this piece is. Values higher than `1.0` lead to distortion
/// * `duration` - The amount of time the piece will be playing for
#[derive(Clone, Debug)]
pub struct Static {
    pub frequency: f64,
    pub amplitude: f64,
}

impl WriteAudio for Static {
    fn write<W: Writeable>(
        &self,
        writer: &mut WavWriter<W>,
        sample_rate: u32,
        sample_count: u64,
    ) -> anyhow::Result<()>
where {
        let coefficient_iter = (0..sample_count).map(|x| x as f64 / f64::from(sample_rate));
        for coefficient in coefficient_iter {
            let base_value = (2.0 * PI * coefficient * self.frequency).cos();
            let sample = base_value * self.amplitude;

            writer.write_sample(sample as f32)?;
        }

        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct Fade {
    pub duration: time::Duration,
    pub end_amplitude: f64,
    pub frequency: f64,
    pub reverse: bool,
}

impl WriteAudio for Fade {
    fn write<W: Writeable>(
        &self,
        writer: &mut WavWriter<W>,
        sample_rate: u32,
        sample_count: u64,
    ) -> anyhow::Result<()> {
        let end_amplitude = self.end_amplitude;
        let fraction = end_amplitude / sample_count as f64;

        let sample_iter = (0..sample_count)
            .map(|x| x as f64 * fraction)
            .map(|part| {
                if self.reverse {
                    part
                } else {
                    end_amplitude - part
                }
            })
            .enumerate()
            .map(|(index, amplitude)| {
                let coefficient = index as f64 / f64::from(sample_rate);
                let base_value = (2.0 * PI * coefficient * self.frequency).cos();
                base_value * amplitude
            });

        for sample in sample_iter {
            writer.write_sample(sample as f32)?;
        }

        Ok(())
    }
}
