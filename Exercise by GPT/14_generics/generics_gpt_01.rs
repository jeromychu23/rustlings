#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Beginner
// Scenario: Data ingestion
//
// Task: Store small signed and unsigned counts in one vector.

fn normalize_counts() -> Vec<i16> {
    // TODO: Push all three input values into `counts`.
    // Hint: `u8` and `i8` can both be converted into `i16`.
    let mut counts: Vec<i16> = Vec::new();

    let open_count: u8 = 12;
    let correction: i8 = -3;
    let retry_count: u8 = 2;

    counts
}

fn main() {
    println!("{:?}", normalize_counts());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_all_counts_as_one_integer_type() {
        assert_eq!(normalize_counts(), vec![12, -3, 2]);
    }
}
