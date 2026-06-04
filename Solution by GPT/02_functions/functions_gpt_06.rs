// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Normalize invalid HTTP status codes into a safe fallback.

fn normalize_status_code(code: u16) -> u16 {
    if code >= 100 && code <= 599 {
        code
    } else {
        500
    }
}

fn main() {
    println!("{}", normalize_status_code(700));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_status_code() {
        assert_eq!(normalize_status_code(200), 200);
        assert_eq!(normalize_status_code(599), 599);
        assert_eq!(normalize_status_code(99), 500);
        assert_eq!(normalize_status_code(700), 500);
    }
}
