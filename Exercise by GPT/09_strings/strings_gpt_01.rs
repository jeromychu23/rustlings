#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Normalize an endpoint path by trimming and adding a leading slash.

fn normalize_path(path: &str) -> String {
    // TODO: Trim whitespace. If the result has no leading '/', add one.
    String::new()
}

fn main() {
    println!("{}", normalize_path(" health "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_paths() {
        assert_eq!(normalize_path(" health "), "/health");
        assert_eq!(normalize_path("/v1/orders"), "/v1/orders");
    }
}
