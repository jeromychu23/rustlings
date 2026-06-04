#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Keep a helper private and expose only the module API.

mod metrics {
    fn percent(part: u32, total: u32) -> u32 {
        // TODO: Return 0 if total is 0, otherwise part * 100 / total.
        0
    }

    pub fn error_percent(errors: u32, requests: u32) -> u32 {
        // TODO: Call the private helper.
        0
    }
}

fn main() {
    println!("{}", metrics::error_percent(5, 100));
}

#[cfg(test)]
mod tests {
    use super::metrics;

    #[test]
    fn computes_error_percent() {
        assert_eq!(metrics::error_percent(5, 100), 5);
        assert_eq!(metrics::error_percent(1, 3), 33);
        assert_eq!(metrics::error_percent(3, 0), 0);
    }
}
