#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Advanced
// Scenario: Generic validation pipeline
//
// Task: Build a small generic pipeline with two trait contracts.

trait Validate {
    fn validate(&self) -> Result<(), String>;
}

trait ToLine {
    fn to_line(&self) -> String;
}

struct WorkOrderRecord {
    id: String,
    aircraft: String,
    hours_since_last_check: u32,
}

impl Validate for WorkOrderRecord {
    fn validate(&self) -> Result<(), String> {
        // TODO: id and aircraft are required.
        // TODO: hours_since_last_check must be at most 500.
        Ok(())
    }
}

impl ToLine for WorkOrderRecord {
    fn to_line(&self) -> String {
        // TODO: Format as "<id>|<aircraft>|<hours_since_last_check>".
        String::new()
    }
}

struct PipelineReport {
    accepted_lines: Vec<String>,
    errors: Vec<String>,
}

impl PipelineReport {
    fn accepted_count(&self) -> usize {
        // TODO: Count accepted rows.
        0
    }

    fn error_count(&self) -> usize {
        // TODO: Count rejected rows.
        0
    }

    fn is_clean(&self) -> bool {
        // TODO: Clean means there are no errors.
        true
    }
}

fn run_pipeline<T: Validate + ToLine>(records: &[T]) -> PipelineReport {
    // TODO: For each record, validate first.
    // TODO: Valid rows become output lines; invalid rows become errors.
    PipelineReport {
        accepted_lines: Vec::new(),
        errors: Vec::new(),
    }
}

fn main() {
    let records = vec![WorkOrderRecord {
        id: String::from("WO-1"),
        aircraft: String::from("B-18301"),
        hours_since_last_check: 12,
    }];
    let report = run_pipeline(&records);
    println!("{}", report.accepted_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_records_and_rejects_invalid_records() {
        let records = vec![
            WorkOrderRecord {
                id: String::from("WO-1"),
                aircraft: String::from("B-18301"),
                hours_since_last_check: 12,
            },
            WorkOrderRecord {
                id: String::new(),
                aircraft: String::from("B-18302"),
                hours_since_last_check: 20,
            },
            WorkOrderRecord {
                id: String::from("WO-3"),
                aircraft: String::from("B-18303"),
                hours_since_last_check: 800,
            },
        ];

        let report = run_pipeline(&records);

        assert_eq!(report.accepted_count(), 1);
        assert_eq!(report.error_count(), 2);
        assert!(!report.is_clean());
        assert_eq!(report.accepted_lines, vec!["WO-1|B-18301|12".to_string()]);
        assert_eq!(
            report.errors,
            vec![
                "work order id is required".to_string(),
                "hours since last check exceeds limit".to_string()
            ]
        );
    }

    #[test]
    fn clean_report_has_no_errors() {
        let records = vec![WorkOrderRecord {
            id: String::from("WO-9"),
            aircraft: String::from("B-18309"),
            hours_since_last_check: 499,
        }];

        let report = run_pipeline(&records);

        assert_eq!(report.accepted_count(), 1);
        assert_eq!(report.error_count(), 0);
        assert!(report.is_clean());
    }
}
