// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Track accepted and rejected requests with mutable counters.

fn request_total(accepted: u32, rejected: u32, timed_out: u32) -> u32 {
    let mut total = accepted;
    total += rejected;
    total += timed_out;
    total
}

fn main() {
    println!("{}", request_total(10, 2, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_request_counts() {
        assert_eq!(request_total(10, 2, 1), 13);
        assert_eq!(request_total(0, 0, 5), 5);
    }
}
