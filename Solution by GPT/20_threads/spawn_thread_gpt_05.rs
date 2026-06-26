// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Advanced
// Scenario: Normalizing task names in parallel
//
// Task: Spawn workers that trim and uppercase task names.

use std::thread;

fn normalize_tasks(tasks: Vec<String>) -> Vec<String> {
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| thread::spawn(move || task.trim().to_ascii_uppercase()))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
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
