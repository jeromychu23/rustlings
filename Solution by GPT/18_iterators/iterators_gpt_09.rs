// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Advanced
// Scenario: Sensor quality summary

#[derive(Debug, Default, PartialEq, Eq)]
struct ReadingSummary {
    observed_count: usize,
    missing_count: usize,
    sum: i64,
    min: Option<i32>,
    max: Option<i32>,
}

fn summarize_readings(readings: &[Option<i32>]) -> ReadingSummary {
    readings
        .iter()
        .fold(ReadingSummary::default(), |mut summary, reading| {
            match reading {
                Some(value) => {
                    summary.observed_count += 1;
                    summary.sum += i64::from(*value);
                    summary.min = Some(summary.min.map_or(*value, |min| min.min(*value)));
                    summary.max = Some(summary.max.map_or(*value, |max| max.max(*value)));
                }
                None => summary.missing_count += 1,
            }
            // The accumulator carries every aggregate through the same pass.
            summary
        })
}

fn main() {
    println!("{:?}", summarize_readings(&[Some(12), None, Some(-3)]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_mixed_readings() {
        assert_eq!(
            summarize_readings(&[Some(12), None, Some(-3), Some(20), None]),
            ReadingSummary {
                observed_count: 3,
                missing_count: 2,
                sum: 29,
                min: Some(-3),
                max: Some(20),
            }
        );
    }

    #[test]
    fn handles_all_missing_readings() {
        assert_eq!(
            summarize_readings(&[None, None]),
            ReadingSummary {
                observed_count: 0,
                missing_count: 2,
                sum: 0,
                min: None,
                max: None,
            }
        );
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(summarize_readings(&[]), ReadingSummary::default());
    }

    #[test]
    fn accumulates_i32_extremes_into_i64() {
        assert_eq!(
            summarize_readings(&[Some(i32::MAX), Some(i32::MAX), Some(i32::MIN)]).sum,
            i64::from(i32::MAX) * 2 + i64::from(i32::MIN)
        );
    }
}
