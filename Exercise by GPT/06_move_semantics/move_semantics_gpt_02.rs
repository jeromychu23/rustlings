#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Borrow a payload so the caller can still use it.

fn borrowed_payload_len(payload: &String) -> usize {
    // TODO: Return the length without taking ownership.
    0
}

fn main() {
    let payload = "hello".to_string();
    println!("{}", borrowed_payload_len(&payload));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_payload() {
        let payload = "payload".to_string();
        assert_eq!(borrowed_payload_len(&payload), 7);
        assert_eq!(payload, "payload");
    }
}
