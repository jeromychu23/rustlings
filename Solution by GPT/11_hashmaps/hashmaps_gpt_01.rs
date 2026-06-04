// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Count how many times each status code appears.

use std::collections::HashMap;

fn count_status_codes(codes: &[u16]) -> HashMap<u16, u32> {
    let mut counts = HashMap::new();

    for code in codes {
        let count = counts.entry(*code).or_insert(0);
        *count += 1;
    }

    counts
}

fn main() {
    println!("{:?}", count_status_codes(&[200, 404, 200]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_status_codes() {
        let counts = count_status_codes(&[200, 404, 200, 500, 404]);
        assert_eq!(counts.get(&200), Some(&2));
        assert_eq!(counts.get(&404), Some(&2));
        assert_eq!(counts.get(&500), Some(&1));
    }
}
