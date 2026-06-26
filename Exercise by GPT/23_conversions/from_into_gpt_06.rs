#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Advanced
// Scenario: Fallback conversion versus explicit validation
//
// Task: Keep `From<&str>` infallible with Default fallback, but expose a
// fallible parser through `TryFrom<RawMetric>`.

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
struct Metric {
    name: String,
    value: u16,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            name: "UNKNOWN".to_string(),
            value: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MetricError {
    BadFormat,
    NoName,
    BadValue,
}

struct RawMetric<'a>(&'a str);

fn parse_metric_parts(value: &str) -> Result<(&str, u16), MetricError> {
    // TODO: Parse "name=value"; reject empty names and invalid u16 values.
    Err(MetricError::BadFormat)
}

impl From<&str> for Metric {
    fn from(value: &str) -> Self {
        // TODO: Use parse_metric_parts; invalid input should return Default.
        Self::default()
    }
}

impl<'a> TryFrom<RawMetric<'a>> for Metric {
    type Error = MetricError;

    fn try_from(raw: RawMetric<'a>) -> Result<Self, Self::Error> {
        // TODO: Use parse_metric_parts and preserve the error.
        Err(MetricError::BadFormat)
    }
}

fn main() {
    println!("{:?}", Metric::from("cycles=42"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hides_invalid_input_with_default() {
        assert_eq!(Metric::from("cycles=bad"), Metric::default());
    }

    #[test]
    fn try_from_preserves_invalid_value_error() {
        assert_eq!(
            Metric::try_from(RawMetric("cycles=bad")),
            Err(MetricError::BadValue)
        );
    }

    #[test]
    fn both_paths_accept_valid_input() {
        let expected = Metric {
            name: "cycles".to_string(),
            value: 42,
        };

        assert_eq!(Metric::from("cycles=42"), expected);
        assert_eq!(Metric::try_from(RawMetric("cycles=42")), Ok(expected));
    }
}
