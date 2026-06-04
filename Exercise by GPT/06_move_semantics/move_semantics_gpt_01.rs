#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Consume an owned payload and return its byte length.

fn consume_payload(payload: String) -> usize {
    // TODO: Return the length of payload.
    0
}

fn main() {
    println!("{}", consume_payload("hello".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_payload() {
        assert_eq!(consume_payload("abc".to_string()), 3);
    }
}
