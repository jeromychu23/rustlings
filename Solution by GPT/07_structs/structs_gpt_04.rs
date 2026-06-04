// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Intermediate
// Scenario: Security
//
// Task: Compute remaining quota from a rate limit struct.

struct RateLimit {
    limit: u32,
    used: u32,
}

impl RateLimit {
    fn remaining(&self) -> u32 {
        if self.used >= self.limit {
            0
        } else {
            self.limit - self.used
        }
    }
}

fn main() {
    let limit = RateLimit { limit: 100, used: 25 };
    println!("{}", limit.remaining());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_remaining_quota() {
        assert_eq!(RateLimit { limit: 100, used: 25 }.remaining(), 75);
        assert_eq!(RateLimit { limit: 100, used: 150 }.remaining(), 0);
    }
}
