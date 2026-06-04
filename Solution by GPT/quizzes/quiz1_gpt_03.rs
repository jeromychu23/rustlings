// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Classify a customer by monthly request count.

fn billing_tier(monthly_requests: u32) -> &'static str {
    if monthly_requests < 1_000 {
        "free"
    } else if monthly_requests < 100_000 {
        "team"
    } else {
        "enterprise"
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
