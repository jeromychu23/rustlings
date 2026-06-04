#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 03 If
// Difficulty: Intermediate
// Scenario: Networking
//
// Task: Choose the lower-latency region, preferring primary on ties.

fn choose_region(primary_latency: u32, fallback_latency: u32) -> &'static str {
    // TODO: Return "primary" or "fallback".
    "primary"
}

fn main() {
    println!("{}", choose_region(30, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_region_by_latency() {
        assert_eq!(choose_region(20, 40), "primary");
        assert_eq!(choose_region(50, 30), "fallback");
        assert_eq!(choose_region(10, 10), "primary");
    }
}
