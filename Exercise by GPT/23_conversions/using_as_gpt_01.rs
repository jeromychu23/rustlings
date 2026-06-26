#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Beginner
// Scenario: Average flight hours
//
// Task: Calculate the average flight hours.
// Practice converting `usize` to `f64` before division.

fn average_flight_hours(hours: &[f64]) -> f64 {
    let total: f64 = hours.iter().sum();

    // TODO: Divide by the number of rows.
    // Hint: `hours.len()` is a `usize`, so convert it with `as f64`.
    total
}

fn main() {
    println!("{}", average_flight_hours(&[10.0, 20.0, 30.0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_whole_values() {
        assert_eq!(average_flight_hours(&[10.0, 20.0, 30.0]), 20.0);
    }

    #[test]
    fn averages_fractional_values() {
        let actual = average_flight_hours(&[1.5, 2.5, 5.0]);
        assert!((actual - 3.0).abs() < f64::EPSILON);
    }
}
