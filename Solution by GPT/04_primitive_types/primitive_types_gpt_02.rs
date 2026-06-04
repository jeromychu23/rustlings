// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Beginner
// Scenario: Networking
//
// Task: Sum an array of exactly three ports.

fn port_sum(ports: [u16; 3]) -> u16 {
    ports[0] + ports[1] + ports[2]
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
