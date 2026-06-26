#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Intermediate
// Scenario: Shared completed task list
//
// Task: Use Arc<Mutex<Vec<String>>> to collect task names from workers.

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn collect_completed_tasks(tasks: Vec<&'static str>) -> Vec<String> {
    let completed = Arc::new(Mutex::new(Vec::<String>::new()));

    // TODO: Spawn workers that push completed task names into the shared Vec.
    Vec::new()
}

fn main() {
    println!("{:?}", collect_completed_tasks(vec!["brakes", "oil"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_all_completed_tasks() {
        let mut completed = collect_completed_tasks(vec!["brakes", "oil", "lights"]);
        completed.sort();

        assert_eq!(
            completed,
            vec![
                "brakes".to_string(),
                "lights".to_string(),
                "oil".to_string(),
            ]
        );
    }
}
