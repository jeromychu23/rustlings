#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Advanced
// Scenario: Security
//
// Task: Rotate a token version by rebinding intermediate values.

fn rotate_token_version(version: u32) -> u32 {
    // TODO: Add 1, then multiply by 10 to build the next public version.
    (version + 1) * 10
}

fn main() {
    println!("{}", rotate_token_version(7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_version() {
        assert_eq!(rotate_token_version(0), 10);
        assert_eq!(rotate_token_version(7), 80);
    }
}
