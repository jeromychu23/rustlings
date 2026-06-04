// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Use a mutable variable to count one more retry.

fn increase_retry_count(current: u8) -> u8 {
    let mut retries = current;
    retries += 1;
    retries
}

fn main() {
    println!("{}", increase_retry_count(2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_retry_count() {
        assert_eq!(increase_retry_count(0), 1);
        assert_eq!(increase_retry_count(2), 3);
    }
}
