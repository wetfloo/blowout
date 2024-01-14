use crate::{cli::InputError, regex::UNITS_REGEX};

type MeasurementUnitValue = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementUnit(pub MeasurementUnitValue);

impl TryFrom<String> for MeasurementUnit {
    type Error = InputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(InputError::NoInput);
        }

        let matches = UNITS_REGEX.find(value.as_str()).is_some();
        if matches {
            Ok(MeasurementUnit(value))
        } else {
            Err(InputError::InvalidUnit(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::MeasurementUnit;

    #[test]
    fn test_units_valid() {
        let input = "mil/s".to_string();
        let units: Result<MeasurementUnit, _> = input.clone().try_into();

        assert_eq!(Ok(MeasurementUnit(input)), units);
    }
}
