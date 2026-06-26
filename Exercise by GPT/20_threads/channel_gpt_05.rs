#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Advanced
// Scenario: Aggregating typed maintenance events
//
// Task: Send enum events through a channel and summarize them on the receiver.

use std::{sync::mpsc, thread};

#[derive(Debug, PartialEq, Eq)]
enum MaintenanceEvent {
    Completed { tail_number: String, hours: u32 },
    Deferred { tail_number: String },
}

#[derive(Debug, PartialEq, Eq)]
struct EventSummary {
    completed: usize,
    deferred: usize,
    total_hours: u32,
}

fn summarize_events(events: Vec<MaintenanceEvent>) -> EventSummary {
    let (tx, rx) = mpsc::channel::<MaintenanceEvent>();

    // TODO: Send all events from a producer thread, then aggregate on `rx`.
    EventSummary {
        completed: 0,
        deferred: 0,
        total_hours: 0,
    }
}

fn main() {
    let events = vec![MaintenanceEvent::Completed {
        tail_number: "B-1234".to_string(),
        hours: 3,
    }];
    println!("{:?}", summarize_events(events));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_completed_and_deferred_events() {
        let summary = summarize_events(vec![
            MaintenanceEvent::Completed {
                tail_number: "B-1234".to_string(),
                hours: 3,
            },
            MaintenanceEvent::Deferred {
                tail_number: "B-5678".to_string(),
            },
            MaintenanceEvent::Completed {
                tail_number: "B-9012".to_string(),
                hours: 5,
            },
        ]);

        assert_eq!(
            summary,
            EventSummary {
                completed: 2,
                deferred: 1,
                total_hours: 8,
            }
        );
    }
}
