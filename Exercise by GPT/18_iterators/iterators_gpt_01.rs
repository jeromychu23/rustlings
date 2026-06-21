#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Beginner
// Scenario: Maintenance task queue
//
// Task: Read the first task with `next()`, then collect the remaining tasks.
// Do not use `for`, `while`, or manual index traversal.

fn split_first_task<'a>(tasks: &'a [&'a str]) -> (Option<&'a str>, Vec<&'a str>) {
    // TODO: Create one iterator, call `next()` once, and collect what remains.
    (None, Vec::new())
}

fn main() {
    let tasks = ["inspect brakes", "check oil", "test lights"];
    println!("{:?}", split_first_task(&tasks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_the_first_task_from_the_rest() {
        let tasks = ["inspect brakes", "check oil", "test lights"];

        assert_eq!(
            split_first_task(&tasks),
            (Some("inspect brakes"), vec!["check oil", "test lights"],)
        );
    }

    #[test]
    fn handles_one_task() {
        assert_eq!(
            split_first_task(&["inspect brakes"]),
            (Some("inspect brakes"), vec![])
        );
    }

    #[test]
    fn handles_an_empty_queue() {
        assert_eq!(split_first_task(&[]), (None, vec![]));
    }
}
