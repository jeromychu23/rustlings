// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Intermediate
// Scenario: Flight minutes to decimal hours
//
// Task: Convert minutes to decimal flight hours and whole display hours.

fn minutes_to_decimal_hours(minutes: u32) -> f64 {
    minutes as f64 / 60.0
}

fn whole_hours_for_display(minutes: u32) -> u32 {
    (minutes as f64 / 60.0) as u32
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
