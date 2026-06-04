#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Destructure a tuple containing an endpoint and a status code.

fn status_line(record: (&str, u16)) -> String {
    // TODO: Return "<path> -> <status>".
    String::new()
}

fn main() {
    println!("{}", status_line(("/health", 200)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_status_line() {
        assert_eq!(status_line(("/health", 200)), "/health -> 200");
        assert_eq!(status_line(("/v1/orders", 404)), "/v1/orders -> 404");
    }
}
