#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Beginner
// Scenario: Work order id normalization
//
// Task: Implement `From<&str>` for a simple newtype.

#[derive(Debug, PartialEq, Eq)]
struct WorkOrderId(String);

impl From<&str> for WorkOrderId {
    fn from(value: &str) -> Self {
        // TODO: Trim the input, uppercase it, and store it in WorkOrderId.
        Self(String::new())
    }
}

fn main() {
    let id: WorkOrderId = " wo-42 ".into();
    println!("{id:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_creates_normalized_id() {
        assert_eq!(WorkOrderId::from(" wo-42 "), WorkOrderId("WO-42".to_string()));
    }

    #[test]
    fn into_is_available_after_from_is_implemented() {
        let id: WorkOrderId = "mx-7".into();
        assert_eq!(id, WorkOrderId("MX-7".to_string()));
    }
}
