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
    let mut handles = Vec::new();

    for _ in 0..job_count {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
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
