// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Intermediate
// Scenario: Developer tooling
//
// Task: Recognize characters that separate route segments.

fn is_route_separator(ch: char) -> bool {
    ch == '/' || ch == ':'
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
