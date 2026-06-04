// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Intermediate
// Scenario: Developer tooling
//
// Task: Split a conversion into a helper function and a public formatter.

fn ms_to_seconds(ms: u64) -> u64 {
    ms / 1000
}

fn duration_label(ms: u64) -> String {
    format!("{}s", ms_to_seconds(ms))
}

fn main() {
    println!("{}", duration_label(2500));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_duration_label() {
        assert_eq!(ms_to_seconds(2500), 2);
        assert_eq!(duration_label(2500), "2s");
        assert_eq!(duration_label(30_000), "30s");
    }
}
