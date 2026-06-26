#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Intermediate
// Scenario: Keeping lock scope small
//
// Task: Build the log entry outside the lock, then lock only long enough to push.

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn record_processed_tasks(tasks: Vec<&'static str>) -> Vec<String> {
    let log = Arc::new(Mutex::new(Vec::<String>::new()));

    // TODO: Normalize each task before locking, then push into the shared log.
    Vec::new()
}

fn main() {
    println!("{:?}", record_processed_tasks(vec![" brakes "]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_normalized_entries() {
        let mut log = record_processed_tasks(vec![" brakes ", "oil"]);
        log.sort();

        assert_eq!(
            log,
            vec!["processed:BRAKES".to_string(), "processed:OIL".to_string()]
        );
    }
}
