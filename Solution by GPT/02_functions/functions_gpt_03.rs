// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Build a stable cache key from a user id and resource name.

fn cache_key(user_id: u64, resource: &str) -> String {
    format!("user:{user_id}:{resource}")
}

fn main() {
    println!("{}", cache_key(42, "profile"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_cache_keys() {
        assert_eq!(cache_key(42, "profile"), "user:42:profile");
        assert_eq!(cache_key(7, "settings"), "user:7:settings");
    }
}
