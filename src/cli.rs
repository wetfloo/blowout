use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Path to the input data file
    pub filepath: PathBuf,

    /// Measurement unit to parse from the input file
    #[arg(short, long, default_value = "m/s")]
    pub unit: String,

    /// Multiply wind amounts by this frequency
    #[arg(short, long, default_value_t = 100.0)]
    pub frequency_multiplier: f32,

    /// Duration of every wind sample in the resulting audio file
    #[arg(short, long, default_value_t = 10)]
    pub sample_duration_ms: u64,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum InputError {
    #[error("the following {0} is not a valid unit")]
    InvalidUnit(String),
    #[error("no measurement unit is provided")]
    NoInput,
}
