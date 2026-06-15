#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Convert a service name into an environment variable key.

fn service_env_key(service: &str) -> String {
    // TODO: Uppercase letters and replace '-' with '_'. Prefix with "SERVICE_".
    let new_service = service.to_uppercase().replace("-", "_");
    format!("SERVICE_{new_service}")
}

fn main() {
    println!("{}", service_env_key("order-api"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_env_key() {
        assert_eq!(service_env_key("order-api"), "SERVICE_ORDER_API");
        assert_eq!(service_env_key("cache"), "SERVICE_CACHE");
    }
}
