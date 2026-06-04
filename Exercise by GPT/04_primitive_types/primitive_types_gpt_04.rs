#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Intermediate
// Scenario: Developer tooling
//
// Task: Recognize characters that separate route segments.

fn is_route_separator(ch: char) -> bool {
    // TODO: Return true for '/' or ':'.
    false
}

fn main() {
    println!("{}", is_route_separator('/'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_route_separators() {
        assert!(is_route_separator('/'));
        assert!(is_route_separator(':'));
        assert!(!is_route_separator('-'));
    }
}
