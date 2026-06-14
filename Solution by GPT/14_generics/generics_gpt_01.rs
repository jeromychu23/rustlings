// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Beginner
// Scenario: Data ingestion
//
// Task: Store small signed and unsigned counts in one vector.

fn normalize_counts() -> Vec<i16> {
    let mut counts: Vec<i16> = Vec::new();

    let open_count: u8 = 12;
    let correction: i8 = -3;
    let retry_count: u8 = 2;

    counts.push(open_count.into());
    counts.push(correction.into());
    counts.push(retry_count.into());

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
