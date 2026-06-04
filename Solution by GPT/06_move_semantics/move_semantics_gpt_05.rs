// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Advanced
// Scenario: Networking
//
// Task: Move extra headers into an existing owned header list.

fn merge_headers(mut base: Vec<String>, extra: Vec<String>) -> Vec<String> {
    for header in extra {
        base.push(header);
    }
    base
}

fn main() {
    println!("{:?}", merge_headers(vec!["a".to_string()], vec!["b".to_string()]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_headers() {
        let base = vec!["content-type".to_string()];
        let extra = vec!["trace-id".to_string(), "user-agent".to_string()];
        assert_eq!(
            merge_headers(base, extra),
            vec![
                "content-type".to_string(),
                "trace-id".to_string(),
                "user-agent".to_string()
            ]
        );
    }
}
