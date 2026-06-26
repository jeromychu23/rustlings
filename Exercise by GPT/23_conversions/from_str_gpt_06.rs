#![allow(unused_variables)]

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
        // TODO: Parse "work_order,aircraft_type,finding_count".
        Err(InspectionRowParseError::BadLen {
            expected: 3,
            found: 0,
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
