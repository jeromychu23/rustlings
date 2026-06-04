#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Intermediate
// Scenario: Networking
//
// Task: Choose retry delay based on attempt and health state.

fn retry_delay_ms(attempt: u8, service_healthy: bool) -> u32 {
    // TODO: Healthy services retry after attempt * 100. Unhealthy services retry after attempt * 500.
    0
}

fn main() {
    println!("{}", retry_delay_ms(3, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_retry_delay() {
        assert_eq!(retry_delay_ms(3, true), 300);
        assert_eq!(retry_delay_ms(3, false), 1500);
    }
}
