// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Decide whether deployment is allowed in a UTC hour window.

fn deploy_window(hour_utc: u8, has_incident: bool) -> &'static str {
    if has_incident {
        "blocked"
    } else if hour_utc >= 9 && hour_utc <= 17 {
        "allowed"
    } else {
        "outside_window"
    }
}

fn main() {
    println!("{}", deploy_window(10, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_deploy_window() {
        assert_eq!(deploy_window(10, false), "allowed");
        assert_eq!(deploy_window(17, false), "allowed");
        assert_eq!(deploy_window(18, false), "outside_window");
        assert_eq!(deploy_window(10, true), "blocked");
    }
}
