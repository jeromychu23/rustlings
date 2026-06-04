#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 02 Functions
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Calculate how many pages an API response needs.

fn page_count(total_items: u32, page_size: u32) -> u32 {
    // TODO: Return the ceiling of total_items / page_size.
    0
}

fn main() {
    println!("{}", page_count(101, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_page_count() {
        assert_eq!(page_count(100, 20), 5);
        assert_eq!(page_count(101, 20), 6);
        assert_eq!(page_count(0, 20), 0);
    }
}
