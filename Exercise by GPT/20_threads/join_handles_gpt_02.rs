#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Beginner
// Scenario: Collecting cycle counts
//
// Task: Join every handle and collect each worker's updated cycle count.

use std::thread;

fn collect_cycle_counts(counts: Vec<u32>) -> Vec<u32> {
    // TODO: Spawn one thread per count. Each worker should add 10 cycles.
    Vec::new()
}

fn main() {
    println!("{:?}", collect_cycle_counts(vec![100, 200, 300]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_multiple_handles() {
        assert_eq!(collect_cycle_counts(vec![100, 200, 300]), vec![110, 210, 310]);
    }
}
