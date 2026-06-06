#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Advanced
// Scenario: Developer tooling
//
// Task: Create a compact debug label using formatted output.

fn debug_label(key: &str, value: u32) -> String {
    // TODO: Return: "debug(<key>=<value>)"
    format!("debug({key}={value})")
}

fn main() {
    println!("{}", debug_label("retries", 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_debug_label() {
        assert_eq!(debug_label("retries", 3), "debug(retries=3)");
        assert_eq!(debug_label("queue", 42), "debug(queue=42)");
    }
}
