#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Beginner
// Scenario: Percentage schema narrowing
//
// Task: Convert i16 into Percentage only when value is in 0..=100.

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
struct Percentage(u8);

#[derive(Debug, PartialEq, Eq)]
enum PercentageError {
    OutOfRange,
}

impl TryFrom<i16> for Percentage {
    type Error = PercentageError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        // TODO: Validate range before using `as u8`.
        Ok(Self(value as u8))
    }
}

fn main() {
    println!("{:?}", Percentage::try_from(95));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_percentage_range() {
        assert_eq!(Percentage::try_from(0), Ok(Percentage(0)));
        assert_eq!(Percentage::try_from(100), Ok(Percentage(100)));
    }

    #[test]
    fn rejects_out_of_range_values() {
        assert_eq!(Percentage::try_from(-1), Err(PercentageError::OutOfRange));
        assert_eq!(Percentage::try_from(101), Err(PercentageError::OutOfRange));
    }
}
