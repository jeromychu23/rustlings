// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Intermediate
// Scenario: Networking
//
// Task: Choose retry delay based on attempt and health state.

fn retry_delay_ms(attempt: u8, service_healthy: bool) -> u32 {
    let attempt = attempt as u32;
    if service_healthy {
        attempt * 100
    } else {
        attempt * 500
    }
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
