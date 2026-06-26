// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Intermediate
// Scenario: Parallel inspection labels
//
// Task: Spawn one thread per inspection task and collect the labels.

use std::thread;

fn process_inspections(tasks: Vec<&'static str>) -> Vec<String> {
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| thread::spawn(move || format!("{task}: done")))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

fn main() {
    let tasks = vec!["brakes", "oil", "lights"];
    println!("{:?}", process_inspections(tasks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_all_inspections() {
        assert_eq!(
            process_inspections(vec!["brakes", "oil", "lights"]),
            vec![
                "brakes: done".to_string(),
                "oil: done".to_string(),
                "lights: done".to_string(),
            ]
        );
    }

    #[test]
    fn handles_empty_task_list() {
        assert!(process_inspections(Vec::new()).is_empty());
    }
}
