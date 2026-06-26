#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Intermediate
// Scenario: Closing a channel after all producers finish
//
// Task: Drop the original sender so receiver iteration can stop.

use std::{sync::mpsc, thread};

fn collect_until_closed(groups: Vec<Vec<&'static str>>) -> Vec<String> {
    let (tx, rx) = mpsc::channel::<String>();

    // TODO: Clone `tx` for each group, spawn producers, drop the original `tx`,
    // then collect until the channel closes.
    Vec::new()
}

fn main() {
    println!("{:?}", collect_until_closed(vec![vec!["A"], vec!["B"]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receiver_iteration_stops_when_senders_are_dropped() {
        let mut messages = collect_until_closed(vec![vec!["A", "B"], vec!["C"]]);
        messages.sort();

        assert_eq!(
            messages,
            vec!["A".to_string(), "B".to_string(), "C".to_string()]
        );
    }

    #[test]
    fn works_with_no_groups() {
        assert!(collect_until_closed(Vec::new()).is_empty());
    }
}
