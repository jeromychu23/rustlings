#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Intermediate
// Scenario: Cycle count parser
//
// Task: Wrap `ParseIntError` with `map_err`.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct CycleCount(u32);

#[derive(Debug, PartialEq)]
enum CycleCountParseError {
    Empty,
    ParseInt(ParseIntError),
}

impl FromStr for CycleCount {
    type Err = CycleCountParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // TODO: Reject empty input, parse u32, and wrap parse errors.
        Err(CycleCountParseError::Empty)
    }
}

fn main() {
    println!("{:?}", "1200".parse::<CycleCount>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cycle_count() {
        assert_eq!("1200".parse::<CycleCount>(), Ok(CycleCount(1200)));
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!("   ".parse::<CycleCount>(), Err(CycleCountParseError::Empty));
    }

    #[test]
    fn wraps_parse_error() {
        assert!(matches!(
            "12OO".parse::<CycleCount>(),
            Err(CycleCountParseError::ParseInt(_))
        ));
    }
}
