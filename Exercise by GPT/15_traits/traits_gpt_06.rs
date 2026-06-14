#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Intermediate
// Scenario: Display labels
//
// Task: Format different domain records through one trait.

trait ToLabel {
    fn label(&self) -> String;
}

struct WorkOrder {
    id: String,
    status: String,
}

struct Aircraft {
    tail_number: String,
    fleet: String,
}

impl ToLabel for WorkOrder {
    fn label(&self) -> String {
        // TODO: Format as "WO <id> [<status>]".
        String::new()
    }
}

impl ToLabel for Aircraft {
    fn label(&self) -> String {
        // TODO: Format as "Aircraft <tail_number> (<fleet>)".
        String::new()
    }
}

fn collect_labels<T: ToLabel>(items: &[T]) -> Vec<String> {
    // TODO: Return each item's label.
    Vec::new()
}

fn main() {
    let orders = vec![WorkOrder {
        id: String::from("100"),
        status: String::from("open"),
    }];
    println!("{:?}", collect_labels(&orders));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_work_order_labels() {
        let orders = vec![
            WorkOrder {
                id: String::from("100"),
                status: String::from("open"),
            },
            WorkOrder {
                id: String::from("200"),
                status: String::from("closed"),
            },
        ];

        assert_eq!(
            collect_labels(&orders),
            vec!["WO 100 [open]".to_string(), "WO 200 [closed]".to_string()]
        );
    }

    #[test]
    fn formats_aircraft_label() {
        let aircraft = Aircraft {
            tail_number: String::from("B-18301"),
            fleet: String::from("A330"),
        };

        assert_eq!(aircraft.label(), "Aircraft B-18301 (A330)");
    }
}
