// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Label API events with mixed enum payload shapes.

enum ApiEvent {
    Request { path: String },
    Error(u16),
    Timeout,
}

fn event_label(event: ApiEvent) -> String {
    match event {
        ApiEvent::Request { path } => format!("request:{path}"),
        ApiEvent::Error(code) => format!("error:{code}"),
        ApiEvent::Timeout => "timeout".to_string(),
    }
}

fn main() {
    println!("{}", event_label(ApiEvent::Timeout));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_events() {
        assert_eq!(event_label(ApiEvent::Request { path: "/health".to_string() }), "request:/health");
        assert_eq!(event_label(ApiEvent::Error(500)), "error:500");
        assert_eq!(event_label(ApiEvent::Timeout), "timeout");
    }
}
