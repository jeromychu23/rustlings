#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Beginner
// Scenario: Security
//
// Task: Accept a request only when authentication and quota both pass.

fn should_accept_request(authenticated: bool, under_limit: bool) -> bool {
    // TODO: Return true only when both inputs are true.
    false
}

fn main() {
    println!("{}", should_accept_request(true, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_access() {
        assert!(should_accept_request(true, true));
        assert!(!should_accept_request(true, false));
        assert!(!should_accept_request(false, true));
    }
}
