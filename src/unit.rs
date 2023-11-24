use crate::{input::InputError, regex::UNITS_REGEX};

type MeasurementUnitValue = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementUnit(MeasurementUnitValue);

impl MeasurementUnit {
    pub fn value(self) -> MeasurementUnitValue {
        self.0
    }
}

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
    use crate::{regex::get_regex, unit::MeasurementUnit};

    #[test]
    fn test_regex() {
        let input_speed = "6900";
        let input_unit = "km/h";
        let input = format!(r"1 18,2°C 40,7% {input_speed} {input_unit} 15.06.2023\11:06:11 ");

        let unit = MeasurementUnit(input_unit.to_string());
        let regex = get_regex(&unit);
        let captures = regex.ok().and_then(|regex| regex.captures(&input)).unwrap();
        assert_eq!(1, captures.iter().skip(1).len());
        assert_eq!(
            Some(input_speed),
            captures
                .iter()
                .skip(1)
                .find_map(|capture| capture.map(|c| c.as_str()))
        );
    }

    #[test]
    fn test_regex_fraction() {
        let input_speed = "0,420";
        let input_unit = "km/h";
        let input = format!(r"1 18,2°C 40,7% {input_speed} {input_unit} 15.06.2023\11:06:11 ");

        let unit = MeasurementUnit(input_unit.to_string());
        let regex = get_regex(&unit);
        let captures = regex.ok().and_then(|regex| regex.captures(&input)).unwrap();
        assert_eq!(1, captures.iter().skip(1).len());
        assert_eq!(
            Some(input_speed),
            captures
                .iter()
                .skip(1)
                .find_map(|capture| capture.map(|c| c.as_str()))
        );
    }

    #[test]
    fn test_units_valid() {
        let input = "mil/s".to_string();
        let units: Result<MeasurementUnit, _> = input.clone().try_into();

        assert_eq!(Ok(MeasurementUnit(input)), units);
    }
}
