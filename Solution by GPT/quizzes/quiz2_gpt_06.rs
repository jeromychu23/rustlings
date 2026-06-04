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
        match command {
            HeaderCommand::LowercaseNames => {
                let mut output = Vec::new();
                for (name, value) in headers {
                    output.push((name.to_lowercase(), value));
                }
                output
            }
            HeaderCommand::KeepPrefix(prefix) => {
                let mut output = Vec::new();
                for (name, value) in headers {
                    if name.starts_with(&prefix) {
                        output.push((name, value));
                    }
                }
                output
            }
            HeaderCommand::Add(name, value) => {
                let mut output = headers;
                output.push((name, value));
                output
            }
        }
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
