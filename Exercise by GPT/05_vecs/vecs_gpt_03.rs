#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Keep only successful status codes from a batch.

fn successful_statuses(statuses: Vec<u16>) -> Vec<u16> {
    // TODO: Return only 2xx status codes, preserving order.
    statuses
        .into_iter()
        .filter(|x| *x >= 200 && *x < 300)
        .collect()
}

fn main() {
    println!("{:?}", successful_statuses(vec![200, 404, 204]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_successful_statuses() {
        assert_eq!(
            successful_statuses(vec![200, 404, 204, 500]),
            vec![200, 204]
        );
        assert_eq!(successful_statuses(vec![400, 500]), Vec::<u16>::new());
    }
}
