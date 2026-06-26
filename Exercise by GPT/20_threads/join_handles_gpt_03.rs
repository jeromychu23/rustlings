#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Intermediate
// Scenario: Preserving report order
//
// Task: Join handles in input order so the output order is deterministic.

use std::thread;

fn build_reports(tail_numbers: Vec<&'static str>) -> Vec<String> {
    // TODO: Spawn workers and join handles in the same order they were created.
    Vec::new()
}

fn main() {
    println!("{:?}", build_reports(vec!["B-1234", "B-5678"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_input_order_after_joining() {
        assert_eq!(
            build_reports(vec!["B-1234", "B-5678", "B-9012"]),
            vec![
                "1:B-1234 ready".to_string(),
                "2:B-5678 ready".to_string(),
                "3:B-9012 ready".to_string(),
            ]
        );
    }
}
