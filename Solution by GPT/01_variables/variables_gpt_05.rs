// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Advanced
// Scenario: Security
//
// Task: Rotate a token version by rebinding intermediate values.

fn rotate_token_version(version: u32) -> u32 {
    let version = version + 1;
    let version = version * 10;
    version
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
