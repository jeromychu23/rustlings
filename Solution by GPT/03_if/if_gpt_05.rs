// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Choose a cache policy from method and status code.

fn cache_policy(method: &str, status: u16) -> &'static str {
    if method == "GET" && status >= 200 && status < 300 {
        "cache"
    } else if method == "GET" && status == 404 {
        "cache_negative"
    } else {
        "skip"
    }
}

fn main() {
    println!("{}", cache_policy("GET", 200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_cache_policy() {
        assert_eq!(cache_policy("GET", 200), "cache");
        assert_eq!(cache_policy("GET", 204), "cache");
        assert_eq!(cache_policy("GET", 404), "cache_negative");
        assert_eq!(cache_policy("POST", 200), "skip");
        assert_eq!(cache_policy("GET", 500), "skip");
    }
}
