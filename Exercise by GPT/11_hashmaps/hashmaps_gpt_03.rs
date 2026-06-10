#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Aggregate route hit counts from a log of paths.

use std::collections::HashMap;

fn route_hits(paths: &[&str]) -> HashMap<String, u32> {
    // TODO: Count hits per path.
    let mut table = HashMap::new();
    for &p in paths {
        let count = table.entry(p.to_string()).or_insert(0);
        *count += 1;
    }
    table
}

fn main() {
    println!("{:?}", route_hits(&["/health", "/health"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_route_hits() {
        let hits = route_hits(&["/health", "/orders", "/health"]);
        assert_eq!(hits.get("/health"), Some(&2));
        assert_eq!(hits.get("/orders"), Some(&1));
    }
}
