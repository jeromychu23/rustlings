// GPT Rustlings extension
// Topic: 20 Threads - spawn thread
// Difficulty: Intermediate
// Scenario: Calculating next due hours
//
// Task: Spawn a worker that owns `InspectionInput` and returns `InspectionResult`.

use std::thread::{self, JoinHandle};

#[derive(Debug, PartialEq, Eq)]
struct InspectionInput {
    tail_number: String,
    current_hours: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct InspectionResult {
    tail_number: String,
    next_due_hours: u32,
}

fn calculate_next_due(input: InspectionInput) -> JoinHandle<InspectionResult> {
    thread::spawn(move || InspectionResult {
        tail_number: input.tail_number,
        next_due_hours: input.current_hours + 50,
    })
}

fn main() {
    let handle = calculate_next_due(InspectionInput {
        tail_number: "B-1234".to_string(),
        current_hours: 1_200,
    });
    println!("{:?}", handle.join().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_struct_from_worker() {
        let handle = calculate_next_due(InspectionInput {
            tail_number: "B-1234".to_string(),
            current_hours: 1_200,
        });

        assert_eq!(
            handle.join().unwrap(),
            InspectionResult {
                tail_number: "B-1234".to_string(),
                next_due_hours: 1_250,
            }
        );
    }
}
