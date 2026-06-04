#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Build a route table from path-port pairs.

use std::collections::HashMap;

fn route_table(routes: Vec<(&str, u16)>) -> HashMap<String, u16> {
    // TODO: Insert each pair into a HashMap<String, u16>.
    HashMap::new()
}

fn main() {
    println!("{:?}", route_table(vec![("/health", 8080)]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_route_table() {
        let table = route_table(vec![("/health", 8080), ("/metrics", 9090)]);
        assert_eq!(table.get("/health"), Some(&8080));
        assert_eq!(table.get("/metrics"), Some(&9090));
    }
}
