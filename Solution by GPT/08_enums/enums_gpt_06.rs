// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Advanced
// Scenario: Security
//
// Task: Map authentication decisions to response statuses.

enum AuthDecision {
    Allow,
    Deny(String),
    Challenge,
}

fn auth_status(decision: AuthDecision) -> u16 {
    match decision {
        AuthDecision::Allow => 200,
        AuthDecision::Deny(_) => 403,
        AuthDecision::Challenge => 401,
    }
}

fn main() {
    println!("{}", auth_status(AuthDecision::Allow));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_auth_statuses() {
        assert_eq!(auth_status(AuthDecision::Allow), 200);
        assert_eq!(auth_status(AuthDecision::Deny("missing scope".to_string())), 403);
        assert_eq!(auth_status(AuthDecision::Challenge), 401);
    }
}
