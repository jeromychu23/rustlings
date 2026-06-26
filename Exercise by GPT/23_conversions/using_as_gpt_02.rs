#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Beginner
// Scenario: Completion percentage
//
// Task: Return completed / total as a percentage.
// Avoid integer division before converting to `f64`.

fn completion_percentage(completed: u32, total: u32) -> f64 {
    if total == 0 {
        return 0.0;
    }

    // TODO: Convert both operands to `f64` before division.
    (completed / total) as f64 * 100.0
}

fn main() {
    println!("{:.1}%", completion_percentage(1, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_quarter_completion() {
        assert_eq!(completion_percentage(1, 4), 25.0);
    }

    #[test]
    fn keeps_fractional_percentage() {
        let actual = completion_percentage(2, 3);
        assert!((actual - 66.666_666_666_666_66).abs() < 0.000_001);
    }

    #[test]
    fn avoids_dividing_by_zero() {
        assert_eq!(completion_percentage(10, 0), 0.0);
    }
}
