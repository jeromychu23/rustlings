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

fn validate_channel(value: i16) -> Result<u8, ColorError> {
    u8::try_from(value).map_err(|_| ColorError::IntConversion)
}

fn color_from_channels(values: [i16; 3]) -> Result<Color, ColorError> {
    Ok(Color {
        red: validate_channel(values[0])?,
        green: validate_channel(values[1])?,
        blue: validate_channel(values[2])?,
    })
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = ColorError;

    fn try_from(value: (i16, i16, i16)) -> Result<Self, Self::Error> {
        color_from_channels([value.0, value.1, value.2])
    }
}

impl TryFrom<[i16; 3]> for Color {
    type Error = ColorError;

    fn try_from(value: [i16; 3]) -> Result<Self, Self::Error> {
        color_from_channels(value)
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = ColorError;

    fn try_from(value: &[i16]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(ColorError::BadLen);
        }

        color_from_channels([value[0], value[1], value[2]])
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
