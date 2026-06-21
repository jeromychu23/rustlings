// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: ETL reading validation
//
// Task: Trim and parse every value as u32.
// Return `EmptyValue` for an empty trimmed value and
// `InvalidValue(<trimmed input>)` for another parse failure.
// Collecting into Result must stop at the first error.
// Do not use loops or manual index traversal.

#[derive(Debug, PartialEq, Eq)]
enum ReadingError {
    EmptyValue,
    InvalidValue(String),
}

fn parse_readings(values: &[&str]) -> Result<Vec<u32>, ReadingError> {
    // TODO: The iterator currently produces ParseIntError. Map each item into
    // Result<u32, ReadingError> before collecting.
    values.iter().map(|raw| raw.trim().parse::<u32>()).collect()
}

fn main() {
    println!("{:?}", parse_readings(&["10", " 20 ", "30"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_valid_readings() {
        assert_eq!(parse_readings(&["10", " 20 ", "30"]), Ok(vec![10, 20, 30]));
    }

    #[test]
    fn accepts_empty_input() {
        assert_eq!(parse_readings(&[]), Ok(vec![]));
    }

    #[test]
    fn reports_an_empty_trimmed_value() {
        assert_eq!(
            parse_readings(&["10", "   ", "30"]),
            Err(ReadingError::EmptyValue)
        );
    }

    #[test]
    fn reports_the_first_invalid_value() {
        assert_eq!(
            parse_readings(&["10", "bad", "also-bad"]),
            Err(ReadingError::InvalidValue("bad".to_string()))
        );
    }
}
