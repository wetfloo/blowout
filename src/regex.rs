use once_cell::sync::Lazy;
use regex::Regex;

const UNITS_REGEX_STR: &str = r"\w+/\w+";
/// This is used to validate if the units
/// are entered in the correct format or not.
pub static UNITS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(UNITS_REGEX_STR).unwrap());
