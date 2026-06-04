// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Beginner
// Scenario: Developer tooling
//
// Task: Transform text commands using strings, vectors, move semantics, modules, and enums.

enum TextCommand {
    Upper,
    Trim,
    Prefix(String),
}

mod text_pipeline {
    use super::TextCommand;

    pub fn run(input: Vec<(String, TextCommand)>) -> Vec<String> {
        let mut output = Vec::new();

        for (text, command) in input {
            let transformed = match command {
                TextCommand::Upper => text.to_uppercase(),
                TextCommand::Trim => text.trim().to_string(),
                TextCommand::Prefix(prefix) => prefix + &text,
            };
            output.push(transformed);
        }

        output
    }
}

fn main() {
    let _ = text_pipeline::run(Vec::new());
}

#[cfg(test)]
mod tests {
    use super::{text_pipeline, TextCommand};

    #[test]
    fn transforms_text_commands() {
        let input = vec![
            ("hello".to_string(), TextCommand::Upper),
            ("  api  ".to_string(), TextCommand::Trim),
            ("orders".to_string(), TextCommand::Prefix("svc:".to_string())),
        ];

        assert_eq!(
            text_pipeline::run(input),
            vec!["HELLO".to_string(), "api".to_string(), "svc:orders".to_string()]
        );
    }
}
