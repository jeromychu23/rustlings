// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Intermediate
// Scenario: Technician assignment parsing
//
// Task: Parse "name,age" into a typed struct and return structured errors.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct TechnicianAssignment {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum AssignmentParseError {
    BadLen,
    NoName,
    ParseAge(ParseIntError),
}

impl FromStr for TechnicianAssignment {
    type Err = AssignmentParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut fields = value.split(',');
        let (Some(name), Some(age), None) = (fields.next(), fields.next(), fields.next()) else {
            return Err(AssignmentParseError::BadLen);
        };

        let name = name.trim();
        if name.is_empty() {
            return Err(AssignmentParseError::NoName);
        }

        let age = age
            .trim()
            .parse::<u8>()
            .map_err(AssignmentParseError::ParseAge)?;

        Ok(Self {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    println!("{:?}", "Ada,33".parse::<TechnicianAssignment>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use AssignmentParseError::*;

    #[test]
    fn parses_valid_assignment() {
        assert_eq!(
            "Ada,33".parse::<TechnicianAssignment>(),
            Ok(TechnicianAssignment {
                name: "Ada".to_string(),
                age: 33,
            })
        );
    }

    #[test]
    fn rejects_bad_field_count() {
        assert_eq!("Ada".parse::<TechnicianAssignment>(), Err(BadLen));
        assert_eq!("Ada,33,extra".parse::<TechnicianAssignment>(), Err(BadLen));
    }

    #[test]
    fn rejects_empty_name_before_parsing_age() {
        assert_eq!(",33".parse::<TechnicianAssignment>(), Err(NoName));
    }

    #[test]
    fn wraps_parse_int_error() {
        assert!(matches!(
            "Ada,thirty".parse::<TechnicianAssignment>(),
            Err(ParseAge(_))
        ));
    }
}
