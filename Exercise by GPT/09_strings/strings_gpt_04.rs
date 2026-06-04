#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Intermediate
// Scenario: Security
//
// Task: Redact a bearer token while preserving the scheme.

fn redact_bearer(header: &str) -> String {
    // TODO: If header starts with "Bearer ", return "Bearer ***"; otherwise return header unchanged.
    String::new()
}

fn main() {
    println!("{}", redact_bearer("Bearer secret"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_bearer_token() {
        assert_eq!(redact_bearer("Bearer abc123"), "Bearer ***");
        assert_eq!(redact_bearer("Basic abc123"), "Basic abc123");
    }
}
