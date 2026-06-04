#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Classify an HTTP status code into a broad status class.

fn status_class(status: u16) -> &'static str {
    // TODO: Return "success", "client_error", "server_error", or "other".
    "other"
}

fn main() {
    println!("{}", status_class(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_status_codes() {
        assert_eq!(status_class(200), "success");
        assert_eq!(status_class(404), "client_error");
        assert_eq!(status_class(503), "server_error");
        assert_eq!(status_class(302), "other");
    }
}
