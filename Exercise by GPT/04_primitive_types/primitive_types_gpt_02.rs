#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Beginner
// Scenario: Networking
//
// Task: Sum an array of exactly three ports.

fn port_sum(ports: [u16; 3]) -> u16 {
    // TODO: Return the sum of all three ports.
    0
}

fn main() {
    println!("{}", port_sum([80, 443, 8080]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_ports() {
        assert_eq!(port_sum([1, 2, 3]), 6);
        assert_eq!(port_sum([80, 443, 8080]), 8603);
    }
}
