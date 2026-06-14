#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Advanced
// Scenario: Data quality
//
// Task: Split generic sensor readings into accepted values and missing ids.

struct SensorReading<T> {
    sensor_id: String,
    value: Option<T>,
}

impl<T> SensorReading<T> {
    fn new(sensor_id: &str, value: Option<T>) -> Self {
        // TODO: Store both fields.
        SensorReading {
            sensor_id: String::new(),
            value: None,
        }
    }
}

struct QualitySummary<T> {
    accepted_values: Vec<T>,
    missing_sensor_ids: Vec<String>,
}

impl<T> QualitySummary<T> {
    fn accepted_count(&self) -> usize {
        // TODO: Count accepted values.
        0
    }

    fn missing_count(&self) -> usize {
        // TODO: Count missing values.
        0
    }

    fn is_clean(&self) -> bool {
        // TODO: A clean summary has no missing values.
        true
    }

    fn into_parts(self) -> (Vec<T>, Vec<String>) {
        // TODO: Return both vectors.
        (Vec::new(), Vec::new())
    }
}

fn summarize_readings<T>(readings: Vec<SensorReading<T>>) -> QualitySummary<T> {
    // TODO: Move present values into accepted_values.
    // TODO: Move sensor ids with missing values into missing_sensor_ids.
    QualitySummary {
        accepted_values: Vec::new(),
        missing_sensor_ids: Vec::new(),
    }
}

fn main() {
    let readings = vec![SensorReading::new("EGT-1", Some(650_u16))];
    let summary = summarize_readings(readings);
    println!("{}", summary.accepted_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_numeric_readings() {
        let readings = vec![
            SensorReading::new("EGT-1", Some(650_u16)),
            SensorReading::new("EGT-2", None),
            SensorReading::new("EGT-3", Some(640_u16)),
        ];

        let summary = summarize_readings(readings);
        assert_eq!(summary.accepted_count(), 2);
        assert_eq!(summary.missing_count(), 1);
        assert!(!summary.is_clean());

        let (accepted, missing) = summary.into_parts();
        assert_eq!(accepted, vec![650, 640]);
        assert_eq!(missing, vec!["EGT-2".to_string()]);
    }

    #[test]
    fn summarizes_text_readings() {
        let readings = vec![
            SensorReading::new("status-a", Some("ok".to_string())),
            SensorReading::new("status-b", Some("warn".to_string())),
        ];

        let summary = summarize_readings(readings);
        assert_eq!(summary.accepted_count(), 2);
        assert_eq!(summary.missing_count(), 0);
        assert!(summary.is_clean());
    }
}
