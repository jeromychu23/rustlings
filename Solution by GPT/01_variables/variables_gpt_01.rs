// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Bind a default timeout value and return it.

fn default_timeout_ms() -> u32 {
    let timeout_ms = 30_000;
    timeout_ms
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
