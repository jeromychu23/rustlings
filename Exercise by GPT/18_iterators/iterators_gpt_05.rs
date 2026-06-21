#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: Fleet maintenance task export
//
// Task: Flatten every aircraft's task list into "<tail number>:<task>" labels.
// Preserve aircraft order and task order. Use `flat_map()`.
// Do not use loops or manual index traversal.

#[derive(Debug, PartialEq, Eq)]
struct Aircraft<'a> {
    tail_number: &'a str,
    tasks: Vec<&'a str>,
}

fn flatten_task_labels(fleet: &[Aircraft<'_>]) -> Vec<String> {
    // TODO: Flatten the nested task collections into one output vector.
    Vec::new()
}

fn main() {
    let fleet = [Aircraft {
        tail_number: "B-18301",
        tasks: vec!["brakes", "oil"],
    }];
    println!("{:?}", flatten_task_labels(&fleet));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattens_fleet_tasks_in_order() {
        let fleet = [
            Aircraft {
                tail_number: "B-18301",
                tasks: vec!["brakes", "oil"],
            },
            Aircraft {
                tail_number: "B-18302",
                tasks: vec!["lights"],
            },
        ];

        assert_eq!(
            flatten_task_labels(&fleet),
            ["B-18301:brakes", "B-18301:oil", "B-18302:lights"]
        );
    }

    #[test]
    fn skips_aircraft_with_no_tasks() {
        let fleet = [
            Aircraft {
                tail_number: "B-18301",
                tasks: vec![],
            },
            Aircraft {
                tail_number: "B-18302",
                tasks: vec!["tires"],
            },
        ];

        assert_eq!(flatten_task_labels(&fleet), ["B-18302:tires"]);
    }

    #[test]
    fn handles_an_empty_fleet() {
        assert!(flatten_task_labels(&[]).is_empty());
    }
}
