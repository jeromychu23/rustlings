#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Beginner
// Scenario: Overdue inspection intervals
//
// Task: Keep values strictly greater than `threshold`.
// Use `filter()` and `copied()`. Do not use loops or manual index traversal.

fn overdue_intervals(intervals: &[u32], threshold: u32) -> Vec<u32> {
    // TODO: Filter borrowed values, then collect owned u32 values.
    Vec::new()
}

fn main() {
    println!("{:?}", overdue_intervals(&[80, 100, 140], 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_strictly_overdue_intervals() {
        assert_eq!(overdue_intervals(&[80, 101, 140, 100], 100), [101, 140]);
    }

    #[test]
    fn excludes_the_threshold_boundary() {
        assert!(overdue_intervals(&[100], 100).is_empty());
    }

    #[test]
    fn preserves_order_and_handles_empty_input() {
        assert_eq!(overdue_intervals(&[9, 30, 12], 10), [30, 12]);
        assert!(overdue_intervals(&[], 10).is_empty());
    }
}
