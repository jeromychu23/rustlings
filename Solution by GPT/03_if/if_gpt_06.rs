// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Advanced
// Scenario: Security
//
// Task: Classify login risk with multiple boolean and numeric branches.

fn login_risk(failed_logins: u8, known_ip: bool) -> &'static str {
    if failed_logins >= 5 {
        "block"
    } else if !known_ip && failed_logins >= 2 {
        "challenge"
    } else {
        "allow"
    }
}

fn main() {
    println!("{}", login_risk(2, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_login_risk() {
        assert_eq!(login_risk(5, true), "block");
        assert_eq!(login_risk(2, false), "challenge");
        assert_eq!(login_risk(1, false), "allow");
        assert_eq!(login_risk(3, true), "allow");
    }
}
