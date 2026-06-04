// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Beginner
// Scenario: Networking
//
// Task: Normalize an HTTP header name.

fn normalize_header_name(raw: &str) -> String {
    raw.trim().to_lowercase()
}

fn main() {
    println!("{}", normalize_header_name(" Content-Type "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_header_name() {
        assert_eq!(normalize_header_name(" Content-Type "), "content-type");
        assert_eq!(normalize_header_name("USER-AGENT"), "user-agent");
    }
}
