#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Advanced
// Scenario: CSV-like maintenance row
//
// Task: Parse "tail,ata,hours" and keep schema errors separate from value errors.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct MaintenanceRow {
    tail_number: String,
    ata_code: String,
    flight_hours: u32,
}

#[derive(Debug, PartialEq)]
enum MaintenanceRowParseError {
    BadLen,
    MissingTail,
    MissingAta,
    ParseHours(ParseIntError),
}

impl FromStr for MaintenanceRow {
    type Err = MaintenanceRowParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // TODO: Validate exactly three fields, required text fields, and u32 hours.
        Err(MaintenanceRowParseError::BadLen)
    }
}

fn main() {
    println!("{:?}", "B-1234,32,1500".parse::<MaintenanceRow>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use MaintenanceRowParseError::*;

    #[test]
    fn parses_valid_row() {
        assert_eq!(
            " b-1234 , 32 , 1500 ".parse::<MaintenanceRow>(),
            Ok(MaintenanceRow {
                tail_number: "B-1234".to_string(),
                ata_code: "32".to_string(),
                flight_hours: 1500,
            })
        );
    }

    #[test]
    fn rejects_bad_schema_width() {
        assert_eq!("B-1234,32".parse::<MaintenanceRow>(), Err(BadLen));
        assert_eq!("B-1234,32,1500,extra".parse::<MaintenanceRow>(), Err(BadLen));
    }

    #[test]
    fn rejects_required_text_fields() {
        assert_eq!(",32,1500".parse::<MaintenanceRow>(), Err(MissingTail));
        assert_eq!("B-1234,,1500".parse::<MaintenanceRow>(), Err(MissingAta));
    }

    #[test]
    fn wraps_hours_parse_error() {
        assert!(matches!(
            "B-1234,32,abc".parse::<MaintenanceRow>(),
            Err(ParseHours(_))
        ));
    }
}
