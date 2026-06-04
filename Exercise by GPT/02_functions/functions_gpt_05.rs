#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Calculate exponential retry backoff without using floating point numbers.

fn retry_backoff_ms(base_ms: u64, attempt: u8) -> u64 {
    // TODO: Return base_ms * 2 repeated `attempt` times.
    base_ms
}

fn main() {
    println!("{}", retry_backoff_ms(100, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_backoff() {
        assert_eq!(retry_backoff_ms(100, 0), 100);
        assert_eq!(retry_backoff_ms(100, 1), 200);
        assert_eq!(retry_backoff_ms(100, 3), 800);
    }
}
