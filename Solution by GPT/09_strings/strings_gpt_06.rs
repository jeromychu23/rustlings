// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Convert a service name into an environment variable key.

fn service_env_key(service: &str) -> String {
    let mut key = String::from("SERVICE_");

    for ch in service.chars() {
        if ch == '-' {
            key.push('_');
        } else {
            key.push(ch.to_ascii_uppercase());
        }
    }

    key
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
