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
        if self.hits == 0 {
            0
        } else {
            self.errors * 100 / self.hits
        }
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
