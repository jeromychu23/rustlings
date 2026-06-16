#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Intermediate
// Scenario: Identity comparison
//
// Task: Use `impl Trait` to compare records from different concrete types.

trait Identified {
    fn id(&self) -> &str;
}

struct WorkOrder {
    id: String,
}

struct Defect {
    id: String,
}

impl Identified for WorkOrder {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for Defect {
    fn id(&self) -> &str {
        &self.id
    }
}

fn same_id(left: impl Identified, right: impl Identified) -> bool {
    // TODO: Compare the two ids.
    left.id() == right.id()
}

fn main() {
    let work_order = WorkOrder {
        id: String::from("WO-1"),
    };
    let defect = Defect {
        id: String::from("WO-1"),
    };
    println!("{}", same_id(work_order, defect));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_different_record_types() {
        let work_order = WorkOrder {
            id: String::from("WO-1"),
        };
        let defect = Defect {
            id: String::from("WO-1"),
        };

        assert!(same_id(work_order, defect));
    }

    #[test]
    fn detects_different_ids() {
        let work_order = WorkOrder {
            id: String::from("WO-1"),
        };
        let defect = Defect {
            id: String::from("DF-9"),
        };

        assert!(!same_id(work_order, defect));
    }
}
