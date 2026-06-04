#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Use a constant limit to compute remaining connection slots.

const MAX_CONNECTIONS: u16 = 256;

fn remaining_connections(current: u16) -> u16 {
    // TODO: Return how many slots are left. If current is above the max, return 0.
    current
}

fn main() {
    println!("{}", remaining_connections(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_remaining_connections() {
        assert_eq!(remaining_connections(0), 256);
        assert_eq!(remaining_connections(200), 56);
        assert_eq!(remaining_connections(300), 0);
    }
}
