// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Beginner
// Scenario: Updating a protected counter
//
// Task: Lock a Mutex, increment the value, and return the final value.

use std::sync::Mutex;

fn increment_counter() -> i32 {
    let counter = Mutex::new(0);

    {
        let mut value = counter.lock().unwrap();
        *value += 1;
    }

    *counter.lock().unwrap()
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
