#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Beginner
// Scenario: Shared job counter
//
// Task: Use Arc<Mutex<u32>> so multiple threads can increment one counter.

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn count_completed_jobs(job_count: usize) -> u32 {
    let counter = Arc::new(Mutex::new(0_u32));

    // TODO: Spawn `job_count` workers. Each worker should lock and increment.
    0
}

fn main() {
    println!("{}", count_completed_jobs(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_all_completed_jobs() {
        assert_eq!(count_completed_jobs(10), 10);
    }

    #[test]
    fn handles_zero_jobs() {
        assert_eq!(count_completed_jobs(0), 0);
    }
}
