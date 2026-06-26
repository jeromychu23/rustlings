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

fn checked_axis(value: i16) -> Result<i8, VibrationError> {
    if !(-100..=100).contains(&value) {
        return Err(VibrationError::OutOfRange);
    }

    Ok(value as i8)
}

impl TryFrom<&[i16]> for VibrationSample {
    type Error = VibrationError;

    fn try_from(values: &[i16]) -> Result<Self, Self::Error> {
        if values.len() != 3 {
            return Err(VibrationError::BadLen);
        }

        Ok(Self {
            x: checked_axis(values[0])?,
            y: checked_axis(values[1])?,
            z: checked_axis(values[2])?,
        })
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
