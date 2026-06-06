#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Beginner
// Scenario: CLI tooling
//
// Task: Return the startup line that a small service would print.

fn startup_message(service: &str, port: u16) -> String {
    // TODO: Return: "<service> listening on :<port>"
    format!("{service} listening on :{port}")
}

fn main() {
    println!("{}", startup_message("gateway", 8080));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_startup_message() {
        assert_eq!(
            startup_message("gateway", 8080),
            "gateway listening on :8080"
        );
        assert_eq!(startup_message("auth", 3000), "auth listening on :3000");
    }
}
