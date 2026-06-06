#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Format a JSON-like response preview without using any JSON crate.

fn response_preview(status: u16, body: &str) -> String {
    // TODO: Return: "status=<status>; body=\"<body>\""
    format!("status={status}; body=\"{body}\"")
}

fn main() {
    println!("{}", response_preview(200, "ok"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_response_preview() {
        assert_eq!(response_preview(200, "ok"), "status=200; body=\"ok\"");
        assert_eq!(
            response_preview(404, "missing"),
            "status=404; body=\"missing\""
        );
    }
}
