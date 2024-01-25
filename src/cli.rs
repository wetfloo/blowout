use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Path to the input data file
    pub filepath: PathBuf,

    /// Measurement unit to parse from the input file
    #[arg(short = 'u', long, default_value = "m/s")]
    pub measurement_unit: String,

    /// Multiply wind speeds by this value
    #[arg(short = 'Q', long, default_value_t = 100.0)]
    pub frequency_multiplier: f64,

    /// Add this frequency to all wind speeds
    #[arg(short = 'D', long, default_value_t = 0.0)]
    pub frequency_term: f64,

    /// Duration of every wind sample in the resulting audio file
    #[arg(short, long, default_value_t = 10)]
    pub sample_duration_ms: u64,

    /// Be careful, values above 1.0 may lead to sound distortion
    #[arg(long, default_value_t = 1.0)]
    pub amplitude: f64,

    /// Output file name
    #[arg(short = 'f', long, default_value = "output.wav")]
    pub file_name: String,
}
