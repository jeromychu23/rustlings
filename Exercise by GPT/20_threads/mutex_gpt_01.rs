#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Beginner
// Scenario: Updating a protected counter
//
// Task: Lock a Mutex, increment the value, and return the final value.

use std::sync::Mutex;

fn increment_counter() -> i32 {
    let counter = Mutex::new(0);

    // TODO: Lock the mutex and increment the protected value by 1.
    0
}

fn main() {
    println!("{}", increment_counter());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_value_inside_mutex() {
        assert_eq!(increment_counter(), 1);
    }
}
