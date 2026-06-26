#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Intermediate
// Scenario: Flight minutes to decimal hours
//
// Task: Convert minutes to decimal flight hours and whole display hours.
// This exercise shows where truncation is intentional and where it is a bug.

fn minutes_to_decimal_hours(minutes: u32) -> f64 {
    // TODO: Convert before division so 95 minutes becomes 1.5833...
    (minutes / 60) as f64
}

fn whole_hours_for_display(minutes: u32) -> u32 {
    // TODO: Convert to decimal hours, then truncate intentionally for display.
    minutes
}

fn main() {
    println!("{:.2}", minutes_to_decimal_hours(95));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_decimal_hours() {
        let actual = minutes_to_decimal_hours(95);
        assert!((actual - 1.583_333_333_333_333_3).abs() < 0.000_001);
    }

    #[test]
    fn display_hours_truncate_fraction() {
        assert_eq!(whole_hours_for_display(95), 1);
        assert_eq!(whole_hours_for_display(120), 2);
    }
}
