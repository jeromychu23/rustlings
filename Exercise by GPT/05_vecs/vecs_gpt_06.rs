#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Keep only the latest N events from an event list.

fn latest_events(events: Vec<String>, n: usize) -> Vec<String> {
    // TODO: Return the last n events. If n is larger than the input, return all events.
    if n > events.len() {
        events
    } else {
        events[(events.len() - n)..].to_vec()
    }
}

fn main() {
    println!(
        "{:?}",
        latest_events(vec!["a".to_string(), "b".to_string()], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_latest_events() {
        let events = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(
            latest_events(events, 2),
            vec!["b".to_string(), "c".to_string()]
        );
        assert_eq!(
            latest_events(vec!["x".to_string()], 5),
            vec!["x".to_string()]
        );
    }
}
