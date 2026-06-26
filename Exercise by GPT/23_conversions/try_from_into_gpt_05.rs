#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Advanced
// Scenario: Shared validation path for color channels
//
// Task: Implement tuple, array, and slice conversions while reusing validation.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq, Eq)]
enum ColorError {
    BadLen,
    IntConversion,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = ColorError;

    fn try_from(value: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // TODO: Reuse a shared validation helper.
        Ok(Self {
            red: value.0 as u8,
            green: value.1 as u8,
            blue: value.2 as u8,
        })
    }
}

impl TryFrom<[i16; 3]> for Color {
    type Error = ColorError;

    fn try_from(value: [i16; 3]) -> Result<Self, Self::Error> {
        // TODO: Reuse the tuple or shared channel path.
        Ok(Self {
            red: value[0] as u8,
            green: value[1] as u8,
            blue: value[2] as u8,
        })
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = ColorError;

    fn try_from(value: &[i16]) -> Result<Self, Self::Error> {
        // TODO: Check length, then reuse the same validation path.
        Ok(Self {
            red: 0,
            green: 0,
            blue: 0,
        })
    }
}

fn main() {
    let color: Result<Color, _> = [183, 65, 14].try_into();
    println!("{color:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ColorError::*;

    #[test]
    fn tuple_array_and_slice_accept_valid_color() {
        assert_eq!(
            Color::try_from((183, 65, 14)),
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14,
            })
        );

        let from_array: Result<Color, _> = [183, 65, 14].try_into();
        assert_eq!(
            from_array,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14,
            })
        );

        assert_eq!(
            Color::try_from(&[183, 65, 14][..]),
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14,
            })
        );
    }

    #[test]
    fn rejects_bad_slice_length() {
        assert_eq!(Color::try_from(&[1, 2][..]), Err(BadLen));
    }

    #[test]
    fn rejects_out_of_range_channel() {
        assert_eq!(Color::try_from((256, 65, 14)), Err(IntConversion));
        assert_eq!(Color::try_from([-1, 65, 14]), Err(IntConversion));
    }
}
