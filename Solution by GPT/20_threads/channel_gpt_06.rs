// GPT Rustlings extension
// Topic: 20 Threads - channel
// Difficulty: Advanced
// Scenario: Fan-in worker messages
//
// Task: Multiple workers should send messages into one receiver.

use std::{sync::mpsc, thread};

#[derive(Debug, PartialEq, Eq)]
struct WorkerBatch {
    worker_id: usize,
    tasks: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct WorkerMessage {
    worker_id: usize,
    task: String,
}

fn collect_worker_messages(batches: Vec<WorkerBatch>) -> Vec<WorkerMessage> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for batch in batches {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            for task in batch.tasks {
                tx.send(WorkerMessage {
                    worker_id: batch.worker_id,
                    task,
                })
                .unwrap();
            }
        }));
    }

    drop(tx);

    let messages: Vec<_> = rx.iter().collect();
    for handle in handles {
        handle.join().unwrap();
    }

    messages
}

fn main() {
    let batches = vec![WorkerBatch {
        worker_id: 1,
        tasks: vec!["brakes".to_string()],
    }];
    println!("{:?}", collect_worker_messages(batches));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fans_in_messages_from_all_workers() {
        let mut messages = collect_worker_messages(vec![
            WorkerBatch {
                worker_id: 2,
                tasks: vec!["oil".to_string(), "lights".to_string()],
            },
            WorkerBatch {
                worker_id: 1,
                tasks: vec!["brakes".to_string()],
            },
        ]);

        messages.sort_by(|left, right| {
            left.worker_id
                .cmp(&right.worker_id)
                .then(left.task.cmp(&right.task))
        });

        assert_eq!(
            messages,
            vec![
                WorkerMessage {
                    worker_id: 1,
                    task: "brakes".to_string(),
                },
                WorkerMessage {
                    worker_id: 2,
                    task: "lights".to_string(),
                },
                WorkerMessage {
                    worker_id: 2,
                    task: "oil".to_string(),
                },
            ]
        );
    }
}
