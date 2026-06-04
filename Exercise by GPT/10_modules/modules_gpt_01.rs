#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Expose a public routing helper from a module.

mod routing {
    pub fn route_name(path: &str) -> &'static str {
        // TODO: Return "health" for "/health", "metrics" for "/metrics", otherwise "unknown".
        "unknown"
    }
}

fn main() {
    println!("{}", routing::route_name("/health"));
}

#[cfg(test)]
mod tests {
    use super::routing;

    #[test]
    fn names_routes() {
        assert_eq!(routing::route_name("/health"), "health");
        assert_eq!(routing::route_name("/metrics"), "metrics");
        assert_eq!(routing::route_name("/other"), "unknown");
    }
}
