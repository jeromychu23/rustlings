// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Beginner
// Scenario: Single inspection worker
//
// Task: Spawn one thread and return the message produced by that thread.

use std::thread;

fn run_single_inspection() -> String {
    let handle = thread::spawn(|| "inspection complete".to_string());
    handle.join().unwrap()
}

fn main() {
    println!("{}", run_single_inspection());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_message_from_spawned_thread() {
        assert_eq!(run_single_inspection(), "inspection complete");
    }
}
