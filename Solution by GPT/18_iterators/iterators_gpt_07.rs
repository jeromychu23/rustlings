// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: ETL reading validation

#[derive(Debug, PartialEq, Eq)]
enum ReadingError {
    EmptyValue,
    InvalidValue(String),
}

fn parse_readings(values: &[&str]) -> Result<Vec<u32>, ReadingError> {
    values
        .iter()
        .map(|raw| {
            let value = raw.trim();
            if value.is_empty() {
                Err(ReadingError::EmptyValue)
            } else {
                value
                    .parse::<u32>()
                    .map_err(|_| ReadingError::InvalidValue(value.to_string()))
            }
        })
        // collect() short-circuits on the first Err instead of building a
        // Vec<Result<u32, ReadingError>>.
        .collect()
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
