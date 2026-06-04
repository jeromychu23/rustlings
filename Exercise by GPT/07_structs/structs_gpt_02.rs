#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Build a service config and format its base URL.

struct ServiceConfig {
    host: String,
    port: u16,
    tls: bool,
}

fn base_url(config: ServiceConfig) -> String {
    // TODO: Use http when tls is false, https when tls is true.
    String::new()
}

fn main() {
    let config = ServiceConfig { host: "localhost".to_string(), port: 8080, tls: false };
    println!("{}", base_url(config));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_base_url() {
        let http = ServiceConfig { host: "localhost".to_string(), port: 8080, tls: false };
        let https = ServiceConfig { host: "api.example.com".to_string(), port: 443, tls: true };
        assert_eq!(base_url(http), "http://localhost:8080");
        assert_eq!(base_url(https), "https://api.example.com:443");
    }
}
