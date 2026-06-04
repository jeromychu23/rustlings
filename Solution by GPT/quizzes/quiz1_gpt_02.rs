// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Return a login capacity message from current and maximum user counts.

fn login_capacity(current_users: u32, max_users: u32) -> &'static str {
    if current_users < max_users {
        "open"
    } else {
        "full"
    }
}

fn main() {
    println!("{}", login_capacity(9, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_login_capacity() {
        assert_eq!(login_capacity(9, 10), "open");
        assert_eq!(login_capacity(10, 10), "full");
    }
}
