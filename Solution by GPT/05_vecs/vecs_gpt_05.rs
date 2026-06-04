// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Advanced
// Scenario: Developer tooling
//
// Task: Convert service names into API paths.

fn service_paths(services: Vec<&str>) -> Vec<String> {
    let mut paths = Vec::new();

    for service in services {
        paths.push(format!("/api/{service}"));
    }

    paths
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
