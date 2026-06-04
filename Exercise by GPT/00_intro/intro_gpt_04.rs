#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Intermediate
// Scenario: Networking
//
// Task: Format an endpoint summary line.

fn endpoint_line(method: &str, path: &str) -> String {
    // TODO: Return: "<METHOD> <path>"
    String::new()
}

fn main() {
    println!("{}", endpoint_line("GET", "/health"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_endpoint_line() {
        assert_eq!(endpoint_line("GET", "/health"), "GET /health");
        assert_eq!(endpoint_line("POST", "/v1/orders"), "POST /v1/orders");
    }
}
