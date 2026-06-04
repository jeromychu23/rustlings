#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Calculate request billing with a simple volume discount.

fn request_price(requests: u32) -> u32 {
    // TODO: First 1,000 requests cost 2 units each. Above that, all requests cost 1 unit each.
    0
}

fn main() {
    println!("{}", request_price(1200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_request_price() {
        assert_eq!(request_price(500), 1000);
        assert_eq!(request_price(1000), 2000);
        assert_eq!(request_price(1001), 1001);
    }
}
