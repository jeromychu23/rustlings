#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Apply one API path command to a batch of paths.

enum ApiCommand {
    NormalizePath,
    AppendVersion(String),
    DropEmpty,
}

mod api {
    use super::ApiCommand;

    pub fn transform(paths: Vec<String>, command: ApiCommand) -> Vec<String> {
        // TODO: Apply command to the whole batch.
        Vec::new()
    }
}

fn main() {
    let _ = api::transform(Vec::new(), ApiCommand::DropEmpty);
}

#[cfg(test)]
mod tests {
    use super::{api, ApiCommand};

    #[test]
    fn transforms_paths() {
        assert_eq!(
            api::transform(vec!["health".to_string(), "/metrics".to_string()], ApiCommand::NormalizePath),
            vec!["/health".to_string(), "/metrics".to_string()]
        );
        assert_eq!(
            api::transform(vec!["/users".to_string()], ApiCommand::AppendVersion("/v1".to_string())),
            vec!["/v1/users".to_string()]
        );
        assert_eq!(
            api::transform(vec!["".to_string(), "/ok".to_string()], ApiCommand::DropEmpty),
            vec!["/ok".to_string()]
        );
    }
}
