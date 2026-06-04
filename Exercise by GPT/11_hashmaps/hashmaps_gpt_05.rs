#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Advanced
// Scenario: Security
//
// Task: Look up a service owner with Option and return an owned value.

use std::collections::HashMap;

fn service_owner(owners: &HashMap<String, String>, service: &str) -> Option<String> {
    // TODO: Return Some(owner) when present, otherwise None.
    None
}

fn main() {
    println!("{:?}", service_owner(&HashMap::new(), "api"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_service_owner() {
        let owners = HashMap::from([
            ("api".to_string(), "platform".to_string()),
            ("billing".to_string(), "finance".to_string()),
        ]);

        assert_eq!(service_owner(&owners, "api"), Some("platform".to_string()));
        assert_eq!(service_owner(&owners, "missing"), None);
    }
}
