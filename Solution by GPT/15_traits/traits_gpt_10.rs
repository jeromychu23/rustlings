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
        if self.id.is_empty() {
            Err(String::from("work order id is required"))
        } else if self.aircraft.is_empty() {
            Err(String::from("aircraft is required"))
        } else if self.hours_since_last_check > 500 {
            Err(String::from("hours since last check exceeds limit"))
        } else {
            Ok(())
        }
    }
}

impl ToLine for WorkOrderRecord {
    fn to_line(&self) -> String {
        format!(
            "{}|{}|{}",
            self.id, self.aircraft, self.hours_since_last_check
        )
    }
}

struct PipelineReport {
    accepted_lines: Vec<String>,
    errors: Vec<String>,
}

impl PipelineReport {
    fn accepted_count(&self) -> usize {
        self.accepted_lines.len()
    }

    fn error_count(&self) -> usize {
        self.errors.len()
    }

    fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
}

fn run_pipeline<T: Validate + ToLine>(records: &[T]) -> PipelineReport {
    let mut accepted_lines = Vec::new();
    let mut errors = Vec::new();

    for record in records {
        match record.validate() {
            Ok(()) => accepted_lines.push(record.to_line()),
            Err(error) => errors.push(error),
        }
    }

    PipelineReport {
        accepted_lines,
        errors,
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
