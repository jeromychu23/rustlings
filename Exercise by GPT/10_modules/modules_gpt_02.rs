#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Use a private module constant inside a public function.

mod config {
    const DEFAULT_PORT: u16 = 8080;

    pub fn service_url(host: &str) -> String {
        // TODO: Return "http://<host>:8080".
        String::new()
    }
}

fn main() {
    println!("{}", config::service_url("localhost"));
}

#[cfg(test)]
mod tests {
    use super::config;

    #[test]
    fn builds_service_url() {
        assert_eq!(config::service_url("localhost"), "http://localhost:8080");
    }
}
