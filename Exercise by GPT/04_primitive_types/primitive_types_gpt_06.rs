#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Find the largest payload size in a slice.

fn max_payload_size(sizes: &[usize]) -> usize {
    // TODO: Return 0 for an empty slice, otherwise return the largest size.
    0
}

fn main() {
    println!("{}", max_payload_size(&[10, 2048, 512]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_payload_size() {
        assert_eq!(max_payload_size(&[]), 0);
        assert_eq!(max_payload_size(&[10, 2048, 512]), 2048);
        assert_eq!(max_payload_size(&[7]), 7);
    }
}
