#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Beginner
// Scenario: Dispatching a work order
//
// Task: Move an owned work order id into a spawned thread.

use std::thread::{self, JoinHandle};

fn spawn_work_order(work_order: String) -> JoinHandle<String> {
    // TODO: Use a `move` closure so the thread owns `work_order`.
    thread::spawn(|| "TODO: dispatch work order".to_string())
}

fn main() {
    let handle = spawn_work_order("WO-100".to_string());
    println!("{}", handle.join().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_work_order_into_thread() {
        let handle = spawn_work_order("WO-100".to_string());

        assert_eq!(handle.join().unwrap(), "WO-100: dispatched");
    }
}
