use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, value_name = "unit")]
    pub unit: Option<String>,
    #[arg(short, long, value_name = "separator")]
    pub separator: Option<String>,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum InputError {
    #[error("the following {0} is not a valid unit")]
    InvalidUnit(String),
    #[error("no measurement unit is provided")]
    NoInput,
}

