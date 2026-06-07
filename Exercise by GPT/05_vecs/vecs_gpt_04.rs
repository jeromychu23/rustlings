#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Double every retry budget in place.

fn double_retry_budgets(budgets: &mut Vec<u8>) {
    // TODO: Multiply every element by 2.
    for budget in budgets {
        *budget *= 2;
    }
}

fn main() {
    let mut budgets = vec![1, 2, 3];
    double_retry_budgets(&mut budgets);
    println!("{budgets:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_budgets() {
        let mut budgets = vec![1, 2, 3];
        double_retry_budgets(&mut budgets);
        assert_eq!(budgets, vec![2, 4, 6]);
    }
}
