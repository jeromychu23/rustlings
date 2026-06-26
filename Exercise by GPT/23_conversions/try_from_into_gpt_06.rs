#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - try_from_into
// Difficulty: Advanced
// Scenario: Raw cycle rows to validated domain rows
//
// Task: Convert a batch of raw rows into domain rows, stopping on the first error.

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
struct RawCycleRow {
    row_number: usize,
    tail_number: String,
    cycles: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct CycleRow {
    tail_number: String,
    cycles: u16,
}

#[derive(Debug, PartialEq, Eq)]
enum CycleRowError {
    EmptyTail { row_number: usize },
    CyclesOutOfRange { row_number: usize, value: i32 },
}

impl TryFrom<RawCycleRow> for CycleRow {
    type Error = CycleRowError;

    fn try_from(raw: RawCycleRow) -> Result<Self, Self::Error> {
        // TODO: Validate tail_number and u16 cycle range.
        Err(CycleRowError::EmptyTail {
            row_number: raw.row_number,
        })
    }
}

fn convert_rows(rows: Vec<RawCycleRow>) -> Result<Vec<CycleRow>, CycleRowError> {
    // TODO: Convert every row and stop on the first error.
    Ok(Vec::new())
}

fn main() {
    let rows = vec![RawCycleRow {
        row_number: 1,
        tail_number: "b-1234".to_string(),
        cycles: 1200,
    }];
    println!("{:?}", convert_rows(rows));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_valid_rows() {
        let rows = vec![
            RawCycleRow {
                row_number: 1,
                tail_number: " b-1234 ".to_string(),
                cycles: 1200,
            },
            RawCycleRow {
                row_number: 2,
                tail_number: "b-5678".to_string(),
                cycles: 1400,
            },
        ];

        assert_eq!(
            convert_rows(rows),
            Ok(vec![
                CycleRow {
                    tail_number: "B-1234".to_string(),
                    cycles: 1200,
                },
                CycleRow {
                    tail_number: "B-5678".to_string(),
                    cycles: 1400,
                },
            ])
        );
    }

    #[test]
    fn reports_empty_tail_with_row_number() {
        let rows = vec![RawCycleRow {
            row_number: 7,
            tail_number: " ".to_string(),
            cycles: 1200,
        }];

        assert_eq!(convert_rows(rows), Err(CycleRowError::EmptyTail { row_number: 7 }));
    }

    #[test]
    fn reports_cycle_range_error_with_row_number() {
        let rows = vec![RawCycleRow {
            row_number: 3,
            tail_number: "B-1234".to_string(),
            cycles: 70_000,
        }];

        assert_eq!(
            convert_rows(rows),
            Err(CycleRowError::CyclesOutOfRange {
                row_number: 3,
                value: 70_000,
            })
        );
    }
}
