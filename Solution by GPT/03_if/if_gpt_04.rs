// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Decide what to do with a canary deployment.

fn deployment_action(error_rate_percent: u8, canary_percent: u8) -> &'static str {
    if error_rate_percent >= 10 {
        "rollback"
    } else if canary_percent < 100 {
        "continue_canary"
    } else {
        "promote"
    }
}

fn main() {
    println!("{}", deployment_action(2, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_deployment_action() {
        assert_eq!(deployment_action(12, 10), "rollback");
        assert_eq!(deployment_action(2, 25), "continue_canary");
        assert_eq!(deployment_action(0, 100), "promote");
    }
}
