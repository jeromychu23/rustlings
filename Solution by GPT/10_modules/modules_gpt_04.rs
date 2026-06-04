// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Keep a helper private and expose only the module API.

mod metrics {
    fn percent(part: u32, total: u32) -> u32 {
        if total == 0 {
            0
        } else {
            part * 100 / total
        }
    }

    pub fn error_percent(errors: u32, requests: u32) -> u32 {
        percent(errors, requests)
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
