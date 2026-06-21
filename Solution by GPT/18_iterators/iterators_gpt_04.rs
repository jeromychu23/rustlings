// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Intermediate
// Scenario: Work-order status report

fn numbered_work_orders(work_orders: &[&str], statuses: &[&str]) -> Vec<String> {
    work_orders
        .iter()
        .zip(statuses)
        .enumerate()
        .map(|(index, (work_order, status))| format!("{}: {work_order} [{status}]", index + 1))
        .collect()
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
