// GPT Rustlings extension
// Topic: 23 Conversions - from_str
// Difficulty: Beginner
// Scenario: Work status parsing
//
// Task: Parse a status enum from user or pipeline text.

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum WorkStatus {
    Open,
    Closed,
    Deferred,
}

#[derive(Debug, PartialEq, Eq)]
struct WorkStatusParseError;

impl FromStr for WorkStatus {
    type Err = WorkStatusParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "open" => Ok(Self::Open),
            "closed" => Ok(Self::Closed),
            "deferred" => Ok(Self::Deferred),
            _ => Err(WorkStatusParseError),
        }
    }
}

fn main() {
    println!("{:?}", "open".parse::<WorkStatus>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_case_insensitively() {
        assert_eq!("open".parse::<WorkStatus>(), Ok(WorkStatus::Open));
        assert_eq!("CLOSED".parse::<WorkStatus>(), Ok(WorkStatus::Closed));
        assert_eq!(" Deferred ".parse::<WorkStatus>(), Ok(WorkStatus::Deferred));
    }

    #[test]
    fn rejects_unknown_status() {
        assert_eq!("cancelled".parse::<WorkStatus>(), Err(WorkStatusParseError));
    }
}
