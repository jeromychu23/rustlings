#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Return a login capacity message from current and maximum user counts.

fn login_capacity(current_users: u32, max_users: u32) -> &'static str {
    // TODO: Return "open" if current_users is less than max_users, otherwise "full".
    "full"
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
