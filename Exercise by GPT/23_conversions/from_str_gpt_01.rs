#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Beginner
// Scenario: Aircraft type parsing
//
// Task: Implement `FromStr` so `"A320".parse::<AircraftType>()` works.

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum AircraftType {
    A320,
    B737,
    B787,
}

#[derive(Debug, PartialEq, Eq)]
struct AircraftTypeParseError;

impl FromStr for AircraftType {
    type Err = AircraftTypeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // TODO: Trim and parse A320, B737, or B787.
        Err(AircraftTypeParseError)
    }
}

fn main() {
    println!("{:?}", "A320".parse::<AircraftType>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_aircraft_types() {
        assert_eq!("A320".parse::<AircraftType>(), Ok(AircraftType::A320));
        assert_eq!(" b737 ".parse::<AircraftType>(), Ok(AircraftType::B737));
    }

    #[test]
    fn rejects_unknown_aircraft_type() {
        assert_eq!("A380".parse::<AircraftType>(), Err(AircraftTypeParseError));
    }
}
