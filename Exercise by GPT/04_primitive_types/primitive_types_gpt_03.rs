#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Compute the average latency from a slice.

fn average_latency(latencies: &[u32]) -> u32 {
    // TODO: Return 0 for an empty slice. Otherwise return the whole-number average.
    if latencies.is_empty() {
        0
    } else {
        latencies.iter().sum::<u32>() / latencies.len() as u32
    }
}

fn main() {
    println!("{}", average_latency(&[10, 20, 30]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_latency_values() {
        assert_eq!(average_latency(&[]), 0);
        assert_eq!(average_latency(&[10, 20, 30]), 20);
        assert_eq!(average_latency(&[5, 6]), 5);
    }
}
