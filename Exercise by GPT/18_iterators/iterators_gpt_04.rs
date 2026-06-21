#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: Work-order status report
//
// Task: Pair work orders with statuses, then add a one-based row number.
// Format each row as "<number>: <work order> [<status>]".
// `zip()` must stop at the shorter input. Use `zip()` and `enumerate()`.
// Do not use loops or manual index traversal.

fn numbered_work_orders(work_orders: &[&str], statuses: &[&str]) -> Vec<String> {
    // TODO: Pair, number, format, and collect the rows.
    Vec::new()
}

fn main() {
    println!(
        "{:?}",
        numbered_work_orders(&["WO-10", "WO-11"], &["OPEN", "DONE"])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_and_numbers_rows() {
        assert_eq!(
            numbered_work_orders(&["WO-10", "WO-11"], &["OPEN", "DONE"]),
            ["1: WO-10 [OPEN]", "2: WO-11 [DONE]"]
        );
    }

    #[test]
    fn stops_at_the_shorter_input() {
        assert_eq!(
            numbered_work_orders(&["WO-10", "WO-11", "WO-12"], &["OPEN", "DONE"]),
            ["1: WO-10 [OPEN]", "2: WO-11 [DONE]"]
        );
        assert_eq!(
            numbered_work_orders(&["WO-10"], &["OPEN", "DONE"]),
            ["1: WO-10 [OPEN]"]
        );
    }

    #[test]
    fn handles_empty_input() {
        assert!(numbered_work_orders(&[], &["OPEN"]).is_empty());
        assert!(numbered_work_orders(&["WO-10"], &[]).is_empty());
    }
}
