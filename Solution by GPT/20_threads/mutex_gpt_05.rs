// GPT Rustlings extension
// Topic: 20 Threads - Mutex
// Difficulty: Advanced
// Scenario: Recovering from a poisoned lock
//
// Task: Read the value even if the mutex was poisoned by a panicking worker.

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn recover_counter(counter: &Mutex<i32>) -> i32 {
    match counter.lock() {
        Ok(value) => *value,
        Err(poisoned) => *poisoned.into_inner(),
    }
}

fn main() {
    let counter = Mutex::new(5);
    println!("{}", recover_counter(&counter));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_normal_mutex_value() {
        let counter = Mutex::new(5);

        assert_eq!(recover_counter(&counter), 5);
    }

    #[test]
    fn recovers_value_from_poisoned_mutex() {
        let counter = Arc::new(Mutex::new(7));
        let poisoned_counter = Arc::clone(&counter);

        let _ = thread::spawn(move || {
            let mut value = poisoned_counter.lock().unwrap();
            *value += 1;
            panic!("worker failed while holding the lock");
        })
        .join();

        assert_eq!(recover_counter(&counter), 8);
    }
}
