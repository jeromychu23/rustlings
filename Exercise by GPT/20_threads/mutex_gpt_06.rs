#![allow(unused_variables, unused_imports)]

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
    let counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    // TODO: Spawn one worker per status and increment the shared HashMap.
    HashMap::new()
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
