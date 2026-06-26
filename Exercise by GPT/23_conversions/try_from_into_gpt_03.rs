#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Intermediate
// Scenario: N1 snapshot from tuple and array
//
// Task: Convert two signed readings into u8 values for left/right engine N1.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq)]
struct N1Snapshot {
    left: u8,
    right: u8,
}

#[derive(Debug, PartialEq, Eq)]
enum SnapshotError {
    OutOfRange,
}

impl TryFrom<(i16, i16)> for N1Snapshot {
    type Error = SnapshotError;

    fn try_from(value: (i16, i16)) -> Result<Self, Self::Error> {
        // TODO: Reuse validation for both fields.
        Ok(Self {
            left: value.0 as u8,
            right: value.1 as u8,
        })
    }
}

impl TryFrom<[i16; 2]> for N1Snapshot {
    type Error = SnapshotError;

    fn try_from(value: [i16; 2]) -> Result<Self, Self::Error> {
        // TODO: Convert array by reusing tuple implementation.
        Ok(Self {
            left: value[0] as u8,
            right: value[1] as u8,
        })
    }
}

fn main() {
    let snapshot: Result<N1Snapshot, _> = [88, 91].try_into();
    println!("{snapshot:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_converts_valid_snapshot() {
        assert_eq!(
            N1Snapshot::try_from((88, 91)),
            Ok(N1Snapshot { left: 88, right: 91 })
        );
    }

    #[test]
    fn array_converts_valid_snapshot() {
        let snapshot: Result<N1Snapshot, _> = [88, 91].try_into();
        assert_eq!(snapshot, Ok(N1Snapshot { left: 88, right: 91 }));
    }

    #[test]
    fn rejects_out_of_range_values() {
        assert_eq!(
            N1Snapshot::try_from((88, 256)),
            Err(SnapshotError::OutOfRange)
        );
        assert_eq!(
            N1Snapshot::try_from([-1, 90]),
            Err(SnapshotError::OutOfRange)
        );
    }
}
