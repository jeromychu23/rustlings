// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Advanced
// Scenario: Shared status counts
//
// Task: Use Arc<Mutex<HashMap<_, _>>> to count task statuses from workers.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

fn count_statuses(statuses: Vec<&'static str>) -> HashMap<String, u32> {
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for status in statuses {
        let counts = Arc::clone(&counts);
        handles.push(thread::spawn(move || {
            let mut counts = counts.lock().unwrap();
            *counts.entry(status.to_string()).or_insert(0) += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(counts).unwrap().into_inner().unwrap()
}

fn main() {
    println!("{:?}", count_statuses(vec!["open", "closed", "open"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_statuses_from_multiple_workers() {
        let counts = count_statuses(vec!["open", "closed", "open", "deferred", "closed"]);

        assert_eq!(counts.get("open"), Some(&2));
        assert_eq!(counts.get("closed"), Some(&2));
        assert_eq!(counts.get("deferred"), Some(&1));
        assert_eq!(counts.get("missing"), None);
    }
}
