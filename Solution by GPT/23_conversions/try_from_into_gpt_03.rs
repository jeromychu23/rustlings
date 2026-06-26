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

fn checked_channel(value: i16) -> Result<u8, SnapshotError> {
    if !(0..=100).contains(&value) {
        return Err(SnapshotError::OutOfRange);
    }

    Ok(value as u8)
}

impl TryFrom<(i16, i16)> for N1Snapshot {
    type Error = SnapshotError;

    fn try_from(value: (i16, i16)) -> Result<Self, Self::Error> {
        Ok(Self {
            left: checked_channel(value.0)?,
            right: checked_channel(value.1)?,
        })
    }
}

impl TryFrom<[i16; 2]> for N1Snapshot {
    type Error = SnapshotError;

    fn try_from(value: [i16; 2]) -> Result<Self, Self::Error> {
        Self::try_from((value[0], value[1]))
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
