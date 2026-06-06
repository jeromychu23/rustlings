#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Bind a default timeout value and return it.

fn default_timeout_ms() -> u32 {
    // TODO: Bind the default timeout to a variable, then return it.
    30000
}

fn main() {
    println!("{}", default_timeout_ms());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_default_timeout() {
        assert_eq!(default_timeout_ms(), 30_000);
    }
}
