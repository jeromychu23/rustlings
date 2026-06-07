#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Create a route list for a small service.

fn default_routes() -> Vec<String> {
    // TODO: Return "/health" and "/metrics" as owned Strings.
    vec!["/health".to_string(), "/metrics".to_string()]
}

fn main() {
    println!("{:?}", default_routes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_routes() {
        assert_eq!(
            default_routes(),
            vec!["/health".to_string(), "/metrics".to_string()]
        );
    }
}
