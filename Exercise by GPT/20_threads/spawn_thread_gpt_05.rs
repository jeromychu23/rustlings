#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Advanced
// Scenario: Normalizing task names in parallel
//
// Task: Spawn workers that trim and uppercase task names.

use std::thread;

fn normalize_tasks(tasks: Vec<String>) -> Vec<String> {
    // TODO: Spawn one thread per task. Each thread should trim whitespace and
    // uppercase the task name. Collect results in the original input order.
    Vec::new()
}

fn main() {
    let tasks = vec![" brakes ".to_string(), "oil".to_string()];
    println!("{:?}", normalize_tasks(tasks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_tasks_in_input_order() {
        let tasks = vec![
            " brakes ".to_string(),
            "oil".to_string(),
            " Landing Gear ".to_string(),
        ];

        assert_eq!(
            normalize_tasks(tasks),
            vec![
                "BRAKES".to_string(),
                "OIL".to_string(),
                "LANDING GEAR".to_string(),
            ]
        );
    }
}
