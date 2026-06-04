// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Compute the average latency from a slice.

fn average_latency(latencies: &[u32]) -> u32 {
    if latencies.is_empty() {
        return 0;
    }

    let mut total = 0;
    for latency in latencies {
        total += latency;
    }

    total / latencies.len() as u32
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
