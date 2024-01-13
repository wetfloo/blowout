pub mod piece;

use std::path::Path;

use anyhow;
use hound::{SampleFormat, WavSpec, WavWriter};

use self::piece::WriteAudio;
pub use self::piece::{Fade, Static};

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
