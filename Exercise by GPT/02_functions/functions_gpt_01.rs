#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Beginner
// Scenario: Networking
//
// Task: Return true when a TCP port is in the common HTTP range used by this toy service.

fn is_http_port(port: u16) -> bool {
    // TODO: Return true for 80, 443, 8080, or 8443.
    false
}

fn main() {
    println!("{}", is_http_port(8080));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_http_ports() {
        assert!(is_http_port(80));
        assert!(is_http_port(443));
        assert!(is_http_port(8080));
        assert!(!is_http_port(22));
    }
}
