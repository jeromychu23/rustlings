// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Advanced
// Scenario: Narrowing a schema field
//
// Task: Convert an i16 field to u8 only after validating the range.

fn checked_u8_for_demo(value: i16) -> Option<u8> {
    if !(0..=255).contains(&value) {
        return None;
    }

    Some(value as u8)
}

fn main() {
    println!("{:?}", checked_u8_for_demo(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_values_inside_u8_range() {
        assert_eq!(checked_u8_for_demo(0), Some(0));
        assert_eq!(checked_u8_for_demo(200), Some(200));
        assert_eq!(checked_u8_for_demo(255), Some(255));
    }

    #[test]
    fn rejects_values_outside_u8_range() {
        assert_eq!(checked_u8_for_demo(-1), None);
        assert_eq!(checked_u8_for_demo(256), None);
    }

    #[test]
    fn raw_cast_wraps_and_is_not_validation() {
        assert_eq!(260_i16 as u8, 4);
    }
}
