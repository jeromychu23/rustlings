#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Advanced
// Scenario: Developer tooling
//
// Task: Split a route path into non-empty owned segments.

fn route_segments(path: &str) -> Vec<String> {
    // TODO: Split by '/', skip empty segments, and return owned Strings.
    path.split('/')
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}

fn main() {
    println!("{:?}", route_segments("/api/v1/orders"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_route_segments() {
        assert_eq!(
            route_segments("/api/v1/orders"),
            vec!["api".to_string(), "v1".to_string(), "orders".to_string()]
        );
        assert_eq!(route_segments("//health/"), vec!["health".to_string()]);
    }
}
