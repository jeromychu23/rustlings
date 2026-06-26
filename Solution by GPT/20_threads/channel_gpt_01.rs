// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Beginner
// Scenario: Sending one maintenance event
//
// Task: Send one String through an mpsc channel and receive it back.

use std::sync::mpsc;

fn send_single_event(event: String) -> String {
    let (tx, rx) = mpsc::channel();
    tx.send(event).unwrap();
    rx.recv().unwrap()
}

fn main() {
    println!("{}", send_single_event("brakes complete".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_and_receives_one_event() {
        assert_eq!(
            send_single_event("brakes complete".to_string()),
            "brakes complete"
        );
    }
}
