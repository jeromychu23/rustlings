#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Advanced
// Scenario: Inspection planning
//
// Task: Consume the records and partition them into two ordered vectors.
// A record is due when `remaining_hours <= 0`; all other records are scheduled.
// Use `into_iter().partition()` and do not clone records.
// Do not use loops or manual index traversal.

#[derive(Debug, PartialEq, Eq)]
struct InspectionRecord {
    id: String,
    remaining_hours: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct InspectionBuckets {
    due: Vec<InspectionRecord>,
    scheduled: Vec<InspectionRecord>,
}

fn partition_inspections(records: Vec<InspectionRecord>) -> InspectionBuckets {
    // TODO: Replace the placeholder predicate with the due rule.
    let (due, scheduled) = records.into_iter().partition(|_| false);
    InspectionBuckets { due, scheduled }
}

fn main() {
    let records = vec![InspectionRecord {
        id: "INSP-1".to_string(),
        remaining_hours: -2,
    }];
    println!("{:?}", partition_inspections(records));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, remaining_hours: i32) -> InspectionRecord {
        InspectionRecord {
            id: id.to_string(),
            remaining_hours,
        }
    }

    #[test]
    fn partitions_records_and_preserves_order() {
        let records = vec![
            record("INSP-1", 20),
            record("INSP-2", -5),
            record("INSP-3", 10),
            record("INSP-4", -1),
        ];

        assert_eq!(
            partition_inspections(records),
            InspectionBuckets {
                due: vec![record("INSP-2", -5), record("INSP-4", -1)],
                scheduled: vec![record("INSP-1", 20), record("INSP-3", 10)],
            }
        );
    }

    #[test]
    fn treats_zero_as_due() {
        assert_eq!(
            partition_inspections(vec![record("INSP-0", 0)]),
            InspectionBuckets {
                due: vec![record("INSP-0", 0)],
                scheduled: vec![],
            }
        );
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(
            partition_inspections(vec![]),
            InspectionBuckets {
                due: vec![],
                scheduled: vec![],
            }
        );
    }
}
