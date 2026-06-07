#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Advanced
// Scenario: Developer tooling
//
// Task: Convert service names into API paths.

fn service_paths(services: Vec<&str>) -> Vec<String> {
    // TODO: Convert each service name into "/api/<service>".
    services
        .into_iter()
        .map(|service| format!("/api/{service}"))
        .collect()
}

fn main() {
    println!("{:?}", service_paths(vec!["users", "orders"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_service_paths() {
        assert_eq!(
            service_paths(vec!["users", "orders"]),
            vec!["/api/users".to_string(), "/api/orders".to_string()]
        );
    }
}
