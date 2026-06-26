#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Beginner
// Scenario: Reading one sensor value
//
// Task: Join one thread and return the value it produced.

use std::thread;

fn read_sensor_value() -> u32 {
    let handle = thread::spawn(|| 42);

    // TODO: Use `join()` to get the value from the worker.
    0
}

fn main() {
    println!("{}", read_sensor_value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_single_handle() {
        assert_eq!(read_sensor_value(), 42);
    }
}
