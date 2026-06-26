// GPT Rustlings extension
// Topic: 23 Conversions - using_as
// Difficulty: Advanced
// Scenario: Nullable average in a data pipeline
//
// Task: Return `None` for an empty input instead of producing NaN.

fn average_or_none(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let total: f64 = values.iter().sum();
    Some(total / values.len() as f64)
}

fn main() {
    println!("{:?}", average_or_none(&[100.0, 120.0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_non_empty_input() {
        assert_eq!(average_or_none(&[100.0, 120.0, 140.0]), Some(120.0));
    }

    #[test]
    fn empty_input_is_missing_value() {
        assert_eq!(average_or_none(&[]), None);
    }
}
