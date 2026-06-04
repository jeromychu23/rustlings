#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Find the service with the highest error count.

use std::collections::HashMap;

fn top_error_service(metrics: &HashMap<String, u32>) -> Option<String> {
    // TODO: Return None for an empty map. Otherwise return the service with the largest count.
    None
}

fn main() {
    println!("{:?}", top_error_service(&HashMap::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_top_error_service() {
        let metrics = HashMap::from([
            ("api".to_string(), 5),
            ("worker".to_string(), 9),
            ("cache".to_string(), 2),
        ]);

        assert_eq!(top_error_service(&metrics), Some("worker".to_string()));
        assert_eq!(top_error_service(&HashMap::new()), None);
    }
}
