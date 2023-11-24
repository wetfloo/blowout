use once_cell::sync::{Lazy, OnceCell as SyncOnceCell};
use regex::Regex;

use crate::unit::MeasurementUnit;

const REGEX_STR: &str = r"(\d+(?:,\d+)?)\s*";
/// This is used for capturing actual input values.
static REGEX: SyncOnceCell<Regex> = SyncOnceCell::new();

const UNITS_REGEX_STR: &str = r"\w+/\w+";
/// This is used to validate if the units
/// are entered in the correct format or not.
pub static UNITS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(UNITS_REGEX_STR).unwrap());

pub fn get_regex(unit: &MeasurementUnit) -> anyhow::Result<&'static Regex> {
    let mut buf = REGEX_STR.to_string();
    buf.push_str(&unit.clone().value());
    let regex: Regex = buf.try_into()?;

    Ok(REGEX.get_or_init(|| regex))
}
