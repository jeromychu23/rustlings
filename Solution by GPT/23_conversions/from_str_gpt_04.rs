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
        let value = value.trim();
        if value.is_empty() {
            return Err(CycleCountParseError::Empty);
        }

        let count = value
            .parse::<u32>()
            .map_err(CycleCountParseError::ParseInt)?;

        Ok(Self(count))
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
