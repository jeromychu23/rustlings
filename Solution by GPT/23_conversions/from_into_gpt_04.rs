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
        Self {
            source: raw.source.trim().to_ascii_uppercase(),
            message: raw.message.trim().to_string(),
        }
    }
}

impl From<&str> for DomainEvent {
    fn from(value: &str) -> Self {
        let Some((source, message)) = value.split_once('|') else {
            return Self {
                source: "UNKNOWN".to_string(),
                message: value.trim().to_string(),
            };
        };

        Self {
            source: source.trim().to_ascii_uppercase(),
            message: message.trim().to_string(),
        }
    }
}

fn event_label<T: Into<DomainEvent>>(event: T) -> String {
    let event = event.into();
    format!("{}:{}", event.source, event.message)
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
