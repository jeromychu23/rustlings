// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Advanced
// Scenario: Detailed inspection row parsing
//
// Task: Build detailed errors so schema problems and invalid values are distinct.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum AircraftType {
    A320,
    B737,
    B787,
}

#[derive(Debug, PartialEq, Eq)]
struct InspectionRow {
    work_order: String,
    aircraft_type: AircraftType,
    finding_count: u16,
}

#[derive(Debug, PartialEq)]
enum InspectionRowParseError {
    BadLen { expected: usize, found: usize },
    EmptyField(&'static str),
    InvalidAircraftType(String),
    InvalidFindingCount(ParseIntError),
}

impl FromStr for InspectionRow {
    type Err = InspectionRowParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let fields = value.split(',').collect::<Vec<_>>();
        if fields.len() != 3 {
            return Err(InspectionRowParseError::BadLen {
                expected: 3,
                found: fields.len(),
            });
        }

        let work_order = fields[0].trim();
        if work_order.is_empty() {
            return Err(InspectionRowParseError::EmptyField("work_order"));
        }

        let raw_aircraft_type = fields[1].trim().to_ascii_uppercase();
        let aircraft_type = match raw_aircraft_type.as_str() {
            "A320" => AircraftType::A320,
            "B737" => AircraftType::B737,
            "B787" => AircraftType::B787,
            _ => return Err(InspectionRowParseError::InvalidAircraftType(raw_aircraft_type)),
        };

        let finding_count = fields[2]
            .trim()
            .parse::<u16>()
            .map_err(InspectionRowParseError::InvalidFindingCount)?;

        Ok(Self {
            work_order: work_order.to_ascii_uppercase(),
            aircraft_type,
            finding_count,
        })
    }
}

fn main() {
    println!("{:?}", "WO-1,A320,2".parse::<InspectionRow>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use InspectionRowParseError::*;

    #[test]
    fn parses_valid_inspection_row() {
        assert_eq!(
            " wo-1 , a320 , 2 ".parse::<InspectionRow>(),
            Ok(InspectionRow {
                work_order: "WO-1".to_string(),
                aircraft_type: AircraftType::A320,
                finding_count: 2,
            })
        );
    }

    #[test]
    fn reports_schema_width() {
        assert_eq!(
            "WO-1,A320".parse::<InspectionRow>(),
            Err(BadLen {
                expected: 3,
                found: 2
            })
        );
    }

    #[test]
    fn reports_empty_required_field() {
        assert_eq!(",A320,2".parse::<InspectionRow>(), Err(EmptyField("work_order")));
    }

    #[test]
    fn reports_invalid_aircraft_type() {
        assert_eq!(
            "WO-1,A380,2".parse::<InspectionRow>(),
            Err(InvalidAircraftType("A380".to_string()))
        );
    }

    #[test]
    fn reports_invalid_finding_count() {
        assert!(matches!(
            "WO-1,A320,two".parse::<InspectionRow>(),
            Err(InvalidFindingCount(_))
        ));
    }
}
