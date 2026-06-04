#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Normalize invalid HTTP status codes into a safe fallback.

fn normalize_status_code(code: u16) -> u16 {
    // TODO: Return code when it is between 100 and 599; otherwise return 500.
    code
}

fn main() {
    println!("{}", normalize_status_code(700));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_status_code() {
        assert_eq!(normalize_status_code(200), 200);
        assert_eq!(normalize_status_code(599), 599);
        assert_eq!(normalize_status_code(99), 500);
        assert_eq!(normalize_status_code(700), 500);
    }
}
