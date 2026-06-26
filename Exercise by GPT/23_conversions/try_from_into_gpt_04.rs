#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Intermediate
// Scenario: Vibration vector from a slice
//
// Task: Validate slice length and convert each axis into a compact i8 field.

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
struct VibrationSample {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Debug, PartialEq, Eq)]
enum VibrationError {
    BadLen,
    OutOfRange,
}

impl TryFrom<&[i16]> for VibrationSample {
    type Error = VibrationError;

    fn try_from(values: &[i16]) -> Result<Self, Self::Error> {
        // TODO: Require exactly 3 values and range -100..=100.
        Ok(Self { x: 0, y: 0, z: 0 })
    }
}

fn main() {
    println!("{:?}", VibrationSample::try_from(&[10, -20, 30][..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_three_axis_sample() {
        assert_eq!(
            VibrationSample::try_from(&[10, -20, 30][..]),
            Ok(VibrationSample { x: 10, y: -20, z: 30 })
        );
    }

    #[test]
    fn rejects_bad_length() {
        assert_eq!(VibrationSample::try_from(&[1, 2][..]), Err(VibrationError::BadLen));
        assert_eq!(
            VibrationSample::try_from(&[1, 2, 3, 4][..]),
            Err(VibrationError::BadLen)
        );
    }

    #[test]
    fn rejects_values_outside_domain_range() {
        assert_eq!(
            VibrationSample::try_from(&[10, -101, 30][..]),
            Err(VibrationError::OutOfRange)
        );
    }
}
