#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Intermediate
// Scenario: Alert routing
//
// Task: Use multiple trait bounds.

trait HasSeverity {
    fn severity(&self) -> u8;
}

trait HasMessage {
    fn message(&self) -> &str;
}

struct Alert {
    severity: u8,
    message: String,
}

impl HasSeverity for Alert {
    fn severity(&self) -> u8 {
        self.severity
    }
}

impl HasMessage for Alert {
    fn message(&self) -> &str {
        &self.message
    }
}

fn should_page(item: impl HasSeverity + HasMessage) -> bool {
    // TODO: Page when severity is at least 8 and the message is not empty.
    false
}

fn main() {
    let alert = Alert {
        severity: 9,
        message: String::from("AOG"),
    };
    println!("{}", should_page(alert));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_for_high_severity_alert_with_message() {
        let alert = Alert {
            severity: 9,
            message: String::from("aircraft on ground"),
        };

        assert!(should_page(alert));
    }

    #[test]
    fn does_not_page_for_low_severity_or_empty_message() {
        let low = Alert {
            severity: 3,
            message: String::from("routine"),
        };
        let empty = Alert {
            severity: 10,
            message: String::new(),
        };

        assert!(!should_page(low));
        assert!(!should_page(empty));
    }
}
