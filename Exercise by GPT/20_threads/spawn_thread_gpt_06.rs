#![allow(unused_variables, unused_imports)]

// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Advanced
// Scenario: Worker factory for inspection batches
//
// Task: Return `JoinHandle<WorkerReport>` from a helper function.

use std::thread::{self, JoinHandle};

#[derive(Debug, PartialEq, Eq)]
struct WorkerReport {
    worker_id: usize,
    completed_tasks: usize,
}

fn spawn_inspection_worker(worker_id: usize, tasks: Vec<String>) -> JoinHandle<WorkerReport> {
    // TODO: Spawn a worker that owns `tasks` and reports how many it completed.
    thread::spawn(|| WorkerReport {
        worker_id: 0,
        completed_tasks: 0,
    })
}

fn run_workers(batches: Vec<Vec<String>>) -> Vec<WorkerReport> {
    // TODO: Spawn one worker per batch. Worker ids should start at 1.
    Vec::new()
}

fn main() {
    let reports = run_workers(vec![vec!["brakes".to_string()], vec!["oil".to_string()]]);
    println!("{reports:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_reports_completed_task_count() {
        let handle = spawn_inspection_worker(
            7,
            vec![
                "brakes".to_string(),
                "oil".to_string(),
                "lights".to_string(),
            ],
        );

        assert_eq!(
            handle.join().unwrap(),
            WorkerReport {
                worker_id: 7,
                completed_tasks: 3,
            }
        );
    }

    #[test]
    fn runs_multiple_workers() {
        let reports = run_workers(vec![
            vec!["brakes".to_string(), "oil".to_string()],
            vec!["lights".to_string()],
        ]);

        assert_eq!(
            reports,
            vec![
                WorkerReport {
                    worker_id: 1,
                    completed_tasks: 2,
                },
                WorkerReport {
                    worker_id: 2,
                    completed_tasks: 1,
                },
            ]
        );
    }
}
