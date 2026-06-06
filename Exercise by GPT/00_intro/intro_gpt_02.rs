#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Return a health banner for an API process.

fn health_banner(service: &str) -> String {
    // TODO: Return: "[OK] <service> is ready"
    format!("[OK] {service} is ready")
}

fn main() {
    println!("{}", health_banner("orders"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_health_banner() {
        assert_eq!(health_banner("orders"), "[OK] orders is ready");
        assert_eq!(health_banner("billing"), "[OK] billing is ready");
    }
}
