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
    let Some((name, raw_value)) = value.split_once('=') else {
        return Err(MetricError::BadFormat);
    };

    let name = name.trim();
    if name.is_empty() {
        return Err(MetricError::NoName);
    }

    let value = raw_value
        .trim()
        .parse::<u16>()
        .map_err(|_| MetricError::BadValue)?;

    Ok((name, value))
}

impl From<&str> for Metric {
    fn from(value: &str) -> Self {
        let Ok((name, value)) = parse_metric_parts(value) else {
            return Self::default();
        };

        Self {
            name: name.to_string(),
            value,
        }
    }
}

impl<'a> TryFrom<RawMetric<'a>> for Metric {
    type Error = MetricError;

    fn try_from(raw: RawMetric<'a>) -> Result<Self, Self::Error> {
        let (name, value) = parse_metric_parts(raw.0)?;

        Ok(Self {
            name: name.to_string(),
            value,
        })
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
