// GPT Rustlings extension
// Topic: 20 Threads - join handles
// Difficulty: Advanced
// Scenario: Batch runner with success and failure counts
//
// Task: Join all workers, keep successful hours, and count failed tasks.

use std::thread;

#[derive(Debug, PartialEq, Eq)]
struct Task {
    name: String,
    expected_hours: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct BatchSummary {
    successful_hours: Vec<u32>,
    failed_tasks: usize,
}

fn run_batch(tasks: Vec<Task>) -> BatchSummary {
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            thread::spawn(move || {
                if task.expected_hours < 0 {
                    Err(task.name)
                } else {
                    Ok(task.expected_hours as u32)
                }
            })
        })
        .collect();

    let mut summary = BatchSummary {
        successful_hours: Vec::new(),
        failed_tasks: 0,
    };

    for handle in handles {
        match handle.join().unwrap() {
            Ok(hours) => summary.successful_hours.push(hours),
            Err(_) => summary.failed_tasks += 1,
        }
    }

    summary
}

fn main() {
    let tasks = vec![Task {
        name: "brakes".to_string(),
        expected_hours: 3,
    }];
    println!("{:?}", run_batch(tasks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_successes_and_failures() {
        let summary = run_batch(vec![
            Task {
                name: "brakes".to_string(),
                expected_hours: 3,
            },
            Task {
                name: "invalid".to_string(),
                expected_hours: -1,
            },
            Task {
                name: "lights".to_string(),
                expected_hours: 2,
            },
        ]);

        assert_eq!(
            summary,
            BatchSummary {
                successful_hours: vec![3, 2],
                failed_tasks: 1,
            }
        );
    }
}
