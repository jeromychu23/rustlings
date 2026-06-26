// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Beginner
// Scenario: Producer thread sends events
//
// Task: Spawn one producer thread that sends every event to the receiver.

use std::{sync::mpsc, thread};

fn collect_events(events: Vec<&'static str>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for event in events {
            tx.send(event.to_string()).unwrap();
        }
    });

    let received: Vec<_> = rx.iter().collect();
    handle.join().unwrap();
    received
}

fn main() {
    println!("{:?}", collect_events(vec!["brakes", "oil"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receives_all_events_from_producer_thread() {
        assert_eq!(
            collect_events(vec!["brakes", "oil", "lights"]),
            vec![
                "brakes".to_string(),
                "oil".to_string(),
                "lights".to_string(),
            ]
        );
    }

    #[test]
    fn handles_no_events() {
        assert!(collect_events(Vec::new()).is_empty());
    }
}
