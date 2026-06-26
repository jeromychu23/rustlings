#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Beginner
// Scenario: Engine cycle count
//
// Task: Convert signed raw cycle count into an unsigned domain type.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq)]
struct EngineCycle(u32);

#[derive(Debug, PartialEq, Eq)]
enum EngineCycleError {
    Negative,
}

impl TryFrom<i32> for EngineCycle {
    type Error = EngineCycleError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        // TODO: Reject negative values, then convert to u32.
        Ok(Self(value as u32))
    }
}

fn main() {
    let cycle: Result<EngineCycle, _> = 1200_i32.try_into();
    println!("{cycle:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_negative_cycles() {
        let cycle: Result<EngineCycle, _> = 1200_i32.try_into();
        assert_eq!(cycle, Ok(EngineCycle(1200)));
    }

    #[test]
    fn rejects_negative_cycles() {
        assert_eq!(EngineCycle::try_from(-1), Err(EngineCycleError::Negative));
    }
}
