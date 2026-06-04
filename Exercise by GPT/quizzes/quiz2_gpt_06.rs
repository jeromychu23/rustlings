#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Advanced
// Scenario: Networking
//
// Task: Apply header commands to a vector of owned header pairs.

enum HeaderCommand {
    LowercaseNames,
    KeepPrefix(String),
    Add(String, String),
}

mod headers {
    use super::HeaderCommand;

    pub fn apply(headers: Vec<(String, String)>, command: HeaderCommand) -> Vec<(String, String)> {
        // TODO: Apply the command to the headers.
        headers
    }
}

fn main() {
    let _ = headers::apply(Vec::new(), HeaderCommand::LowercaseNames);
}

#[cfg(test)]
mod tests {
    use super::{headers, HeaderCommand};

    #[test]
    fn applies_header_commands() {
        let input = vec![("Content-Type".to_string(), "text/plain".to_string())];
        assert_eq!(
            headers::apply(input, HeaderCommand::LowercaseNames),
            vec![("content-type".to_string(), "text/plain".to_string())]
        );

        let input = vec![
            ("x-trace-id".to_string(), "1".to_string()),
            ("content-type".to_string(), "text/plain".to_string()),
        ];
        assert_eq!(
            headers::apply(input, HeaderCommand::KeepPrefix("x-".to_string())),
            vec![("x-trace-id".to_string(), "1".to_string())]
        );

        assert_eq!(
            headers::apply(Vec::new(), HeaderCommand::Add("x-id".to_string(), "42".to_string())),
            vec![("x-id".to_string(), "42".to_string())]
        );
    }
}
