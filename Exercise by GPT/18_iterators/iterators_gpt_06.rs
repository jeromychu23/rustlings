// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: Sensor log parsing
//
// Task: Return the first valid signed temperature from a "TEMP=<value>" line.
// Ignore unrelated lines and malformed temperature values.
// Use `find_map()`. Do not use loops or manual index traversal.

fn first_valid_temperature(lines: &[&str]) -> Option<i32> {
    // TODO: The current iterator returns borrowed text, but this function must
    // return a parsed i32. Update the closure to skip invalid candidates.
    lines.iter().find_map(|line| line.strip_prefix("TEMP="))
}

fn main() {
    println!(
        "{:?}",
        first_valid_temperature(&["INFO=start", "TEMP=bad", "TEMP=-12"])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_first_valid_temperature() {
        let lines = ["INFO=start", "TEMP=bad", "TEMP=-12", "TEMP=25"];
        assert_eq!(first_valid_temperature(&lines), Some(-12));
    }

    #[test]
    fn skips_malformed_candidates() {
        let lines = ["TEMP=", "TEMP=12.5", "TEMP=18"];
        assert_eq!(first_valid_temperature(&lines), Some(18));
    }

    #[test]
    fn returns_none_without_a_valid_temperature() {
        assert_eq!(
            first_valid_temperature(&["INFO=ready", "TEMP=bad", "PRESSURE=30"]),
            None
        );
        assert_eq!(first_valid_temperature(&[]), None);
    }
}
