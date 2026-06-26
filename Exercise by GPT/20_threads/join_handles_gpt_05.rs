#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Advanced
// Scenario: Summing worker hours
//
// Task: Each thread sums one batch, then the main thread joins and totals them.

use std::thread;

fn sum_worker_hours(batches: Vec<Vec<u32>>) -> u32 {
    // TODO: Spawn one thread per batch and sum all joined batch totals.
    0
}

fn main() {
    println!("{}", sum_worker_hours(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_all_worker_results() {
        let batches = vec![vec![10, 20], vec![5, 5, 5], vec![100]];

        assert_eq!(sum_worker_hours(batches), 145);
    }

    #[test]
    fn empty_batches_sum_to_zero() {
        assert_eq!(sum_worker_hours(Vec::new()), 0);
    }
}
