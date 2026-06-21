// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Beginner
// Scenario: Flight-hour display formatting

fn format_flight_hours(hours: &[u32]) -> Vec<String> {
    hours.iter().map(|hours| format!("{hours} FH")).collect()
}

fn main() {
    println!("{:?}", format_flight_hours(&[120, 245]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_each_value() {
        assert_eq!(
            format_flight_hours(&[120, 245, 0]),
            ["120 FH", "245 FH", "0 FH"]
        );
    }

    #[test]
    fn preserves_input_order() {
        assert_eq!(format_flight_hours(&[9, 3]), ["9 FH", "3 FH"]);
    }

    #[test]
    fn handles_empty_input() {
        assert!(format_flight_hours(&[]).is_empty());
    }
}
