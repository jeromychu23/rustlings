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
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for task in tasks {
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            let entry = format!("processed:{}", task.trim().to_ascii_uppercase());
            log.lock().unwrap().push(entry);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(log).unwrap().into_inner().unwrap()
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
