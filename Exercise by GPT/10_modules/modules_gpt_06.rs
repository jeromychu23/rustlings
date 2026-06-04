#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Advanced
// Scenario: Developer tooling
//
// Task: Normalize service names through a nested registry module.

mod services {
    pub mod registry {
        pub fn normalize_name(name: &str) -> String {
            // TODO: Trim and lowercase service names.
            String::new()
        }
    }
}

fn main() {
    println!("{}", services::registry::normalize_name(" Orders "));
}

#[cfg(test)]
mod tests {
    use super::services;

    #[test]
    fn normalizes_service_name() {
        assert_eq!(services::registry::normalize_name(" Orders "), "orders");
        assert_eq!(services::registry::normalize_name("AUTH"), "auth");
    }
}
