#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Intermediate
// Scenario: Generic event API
//
// Task: Use `T: Into<DomainEvent>` so callers can pass different input shapes.

#[derive(Debug, PartialEq, Eq)]
struct RawEvent {
    source: String,
    message: String,
}

#[derive(Debug, PartialEq, Eq)]
struct DomainEvent {
    source: String,
    message: String,
}

impl From<RawEvent> for DomainEvent {
    fn from(raw: RawEvent) -> Self {
        // TODO: Normalize source to uppercase and trim message.
        Self {
            source: raw.source,
            message: raw.message,
        }
    }
}

impl From<&str> for DomainEvent {
    fn from(value: &str) -> Self {
        // TODO: Parse "source|message"; fallback to UNKNOWN when shape is invalid.
        Self {
            source: "UNKNOWN".to_string(),
            message: value.to_string(),
        }
    }
}

fn event_label<T: Into<DomainEvent>>(event: T) -> String {
    // TODO: Convert into DomainEvent and return "SOURCE:message".
    String::new()
}

fn main() {
    println!("{}", event_label("api|created"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_raw_event() {
        let raw = RawEvent {
            source: " api ".to_string(),
            message: " created ".to_string(),
        };

        assert_eq!(event_label(raw), "API:created");
    }

    #[test]
    fn accepts_str_after_from_is_implemented() {
        assert_eq!(event_label("sensor|high vibration"), "SENSOR:high vibration");
    }

    #[test]
    fn accepts_domain_event_directly() {
        let event = DomainEvent {
            source: "PIPELINE".to_string(),
            message: "loaded".to_string(),
        };

        assert_eq!(event_label(event), "PIPELINE:loaded");
    }
}
