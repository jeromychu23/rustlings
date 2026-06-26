#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Intermediate
// Scenario: Sensor reading display
//
// Task: Implement one function that truncates and one function that rounds.
// `as i32` truncates toward zero. It does not round.

fn truncate_reading(value: f64) -> i32 {
    value as i32
}

fn round_reading(value: f64) -> i32 {
    // TODO: Round first, then convert to `i32`.
    value as i32
}

fn main() {
    println!("{}", round_reading(72.8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncation_drops_fraction() {
        assert_eq!(truncate_reading(72.8), 72);
        assert_eq!(truncate_reading(-2.7), -2);
    }

    #[test]
    fn rounding_uses_nearest_integer() {
        assert_eq!(round_reading(72.8), 73);
        assert_eq!(round_reading(-2.7), -3);
    }
}
