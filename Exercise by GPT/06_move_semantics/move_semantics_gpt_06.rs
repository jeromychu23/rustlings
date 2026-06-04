#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Push an owned item through a mutable borrow and report the new queue length.

fn push_and_report(queue: &mut Vec<String>, item: String) -> usize {
    // TODO: Push item and return the new length.
    0
}

fn main() {
    let mut queue = vec!["a".to_string()];
    println!("{}", push_and_report(&mut queue, "b".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_and_reports_len() {
        let mut queue = vec!["a".to_string()];
        let len = push_and_report(&mut queue, "b".to_string());
        assert_eq!(len, 2);
        assert_eq!(queue, vec!["a".to_string(), "b".to_string()]);
    }
}
