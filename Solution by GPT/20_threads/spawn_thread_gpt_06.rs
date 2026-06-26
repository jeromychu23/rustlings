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
    thread::spawn(move || WorkerReport {
        worker_id,
        completed_tasks: tasks.len(),
    })
}

fn run_workers(batches: Vec<Vec<String>>) -> Vec<WorkerReport> {
    let handles: Vec<_> = batches
        .into_iter()
        .enumerate()
        .map(|(index, tasks)| spawn_inspection_worker(index + 1, tasks))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
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
