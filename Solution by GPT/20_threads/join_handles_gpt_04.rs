// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Intermediate
// Scenario: Joining workers that can fail
//
// Task: Convert `JoinHandle<Result<_, _>>` into one flat `Result`.

use std::thread::{self, JoinHandle};

#[derive(Debug, PartialEq, Eq)]
enum WorkerError {
    WorkerFailed(String),
    ThreadPanicked,
}

fn spawn_task(task: String) -> JoinHandle<Result<String, WorkerError>> {
    thread::spawn(move || {
        if task == "blocked" {
            Err(WorkerError::WorkerFailed(task))
        } else {
            Ok(format!("{task}: complete"))
        }
    })
}

fn join_worker(handle: JoinHandle<Result<String, WorkerError>>) -> Result<String, WorkerError> {
    match handle.join() {
        Ok(result) => result,
        Err(_) => Err(WorkerError::ThreadPanicked),
    }
}

fn main() {
    println!("{:?}", join_worker(spawn_task("brakes".to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_successful_worker_value() {
        let handle = spawn_task("brakes".to_string());

        assert_eq!(join_worker(handle), Ok("brakes: complete".to_string()));
    }

    #[test]
    fn returns_worker_error() {
        let handle = spawn_task("blocked".to_string());

        assert_eq!(
            join_worker(handle),
            Err(WorkerError::WorkerFailed("blocked".to_string()))
        );
    }
}
