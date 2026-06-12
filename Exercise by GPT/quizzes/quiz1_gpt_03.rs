#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Classify a customer by monthly request count.

fn billing_tier(monthly_requests: u32) -> &'static str {
    // TODO: < 1_000 => "free", < 100_000 => "team", otherwise "enterprise".
    match monthly_requests {
        0..1000 => "free",
        1000..100_000 => "team",
        _ => "enterprise",
    }
}

fn main() {
    println!("{}", billing_tier(50_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_billing_tier() {
        assert_eq!(billing_tier(999), "free");
        assert_eq!(billing_tier(1_000), "team");
        assert_eq!(billing_tier(100_000), "enterprise");
    }
}
