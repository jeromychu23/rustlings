#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Intermediate
// Scenario: Two maintenance event producers
//
// Task: Clone the sender so two producer threads can send to one receiver.

use std::{sync::mpsc, thread};

struct Queue {
    scheduled: Vec<&'static str>,
    unscheduled: Vec<&'static str>,
}

impl Queue {
    fn new() -> Self {
        Self {
            scheduled: vec!["daily check", "oil service"],
            unscheduled: vec!["bird strike", "tire change"],
        }
    }
}

fn send_from_two_sources(queue: Queue) -> Vec<String> {
    let (tx, rx) = mpsc::channel::<String>();

    // TODO: Clone `tx`, move each sender into one producer, then collect messages.
    Vec::new()
}

fn main() {
    println!("{:?}", send_from_two_sources(Queue::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receives_messages_from_both_producers() {
        let mut messages = send_from_two_sources(Queue::new());
        messages.sort();

        assert_eq!(
            messages,
            vec![
                "scheduled: daily check".to_string(),
                "scheduled: oil service".to_string(),
                "unscheduled: bird strike".to_string(),
                "unscheduled: tire change".to_string(),
            ]
        );
    }
}
