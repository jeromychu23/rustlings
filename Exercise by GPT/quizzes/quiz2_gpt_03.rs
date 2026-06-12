#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Intermediate
// Scenario: CLI tooling
//
// Task: Filter log lines using enum-based filter rules.

enum Filter {
    StartsWith(String),
    Contains(String),
    Exact(String),
}

mod logs {
    use super::Filter;

    pub fn filter(lines: Vec<String>, filter: Filter) -> Vec<String> {
        // TODO: Return lines matching the filter rule.
        let mut lines = lines;
        match &filter {
            Filter::StartsWith(start) => lines.retain(|s| s.starts_with(start)),
            Filter::Contains(contain) => lines.retain(|s| s.contains(contain)),
            Filter::Exact(exact) => lines.retain(|s| s == exact),
        };
        lines
    }
}

fn main() {
    let _ = logs::filter(Vec::new(), Filter::Contains("x".to_string()));
}

#[cfg(test)]
mod tests {
    use super::{Filter, logs};

    #[test]
    fn filters_logs() {
        let lines = vec![
            "INFO boot".to_string(),
            "WARN slow".to_string(),
            "INFO ready".to_string(),
        ];

        assert_eq!(
            logs::filter(lines, Filter::StartsWith("INFO".to_string())),
            vec!["INFO boot".to_string(), "INFO ready".to_string()]
        );

        assert_eq!(
            logs::filter(
                vec!["a=b".to_string(), "c=d".to_string()],
                Filter::Exact("c=d".to_string())
            ),
            vec!["c=d".to_string()]
        );
    }
}
