// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Intermediate
// Scenario: Security
//
// Task: Redact a bearer token while preserving the scheme.

fn redact_bearer(header: &str) -> String {
    if header.starts_with("Bearer ") {
        "Bearer ***".to_string()
    } else {
        header.to_string()
    }
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
