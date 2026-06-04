#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Decide whether deployment is allowed in a UTC hour window.

fn deploy_window(hour_utc: u8, has_incident: bool) -> &'static str {
    // TODO:
    // - if has_incident => "blocked"
    // - if hour is from 9 through 17 inclusive => "allowed"
    // - otherwise => "outside_window"
    "blocked"
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
