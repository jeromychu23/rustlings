#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Calculate error rate percentage from endpoint stats.

struct EndpointStats {
    hits: u32,
    errors: u32,
}

impl EndpointStats {
    fn error_rate_percent(&self) -> u32 {
        // TODO: Return 0 when hits is 0. Otherwise return errors * 100 / hits.
        0
    }
}

fn main() {
    let stats = EndpointStats { hits: 100, errors: 5 };
    println!("{}", stats.error_rate_percent());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_error_rate() {
        assert_eq!(EndpointStats { hits: 100, errors: 5 }.error_rate_percent(), 5);
        assert_eq!(EndpointStats { hits: 3, errors: 1 }.error_rate_percent(), 33);
        assert_eq!(EndpointStats { hits: 0, errors: 10 }.error_rate_percent(), 0);
    }
}
