// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Beginner
// Scenario: Collecting cycle counts
//
// Task: Join every handle and collect each worker's updated cycle count.

use std::thread;

fn collect_cycle_counts(counts: Vec<u32>) -> Vec<u32> {
    let handles: Vec<_> = counts
        .into_iter()
        .map(|count| thread::spawn(move || count + 10))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

fn main() {
    println!("{:?}", collect_cycle_counts(vec![100, 200, 300]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_multiple_handles() {
        assert_eq!(collect_cycle_counts(vec![100, 200, 300]), vec![110, 210, 310]);
    }
}
