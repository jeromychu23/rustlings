// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Calculate exponential retry backoff without using floating point numbers.

fn retry_backoff_ms(base_ms: u64, attempt: u8) -> u64 {
    let mut delay = base_ms;
    let mut remaining = attempt;

    while remaining > 0 {
        delay *= 2;
        remaining -= 1;
    }

    delay
}

fn main() {
    println!("{}", retry_backoff_ms(100, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_backoff() {
        assert_eq!(retry_backoff_ms(100, 0), 100);
        assert_eq!(retry_backoff_ms(100, 1), 200);
        assert_eq!(retry_backoff_ms(100, 3), 800);
    }
}
