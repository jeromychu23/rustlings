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
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for event in events {
            tx.send(event).unwrap();
        }
    });

    let mut summary = EventSummary {
        completed: 0,
        deferred: 0,
        total_hours: 0,
    };

    for event in rx {
        match event {
            MaintenanceEvent::Completed { hours, .. } => {
                summary.completed += 1;
                summary.total_hours += hours;
            }
            MaintenanceEvent::Deferred { .. } => {
                summary.deferred += 1;
            }
        }
    }

    handle.join().unwrap();
    summary
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
