#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 01 Variables
// Difficulty: Intermediate
// Scenario: Developer tooling
//
// Task: Use shadowing to convert bytes to whole kilobytes.

fn whole_kilobytes(bytes: u32) -> u32 {
    // TODO: Shadow `bytes` with its value in whole KiB.
    bytes
}

fn main() {
    println!("{}", whole_kilobytes(4096));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_whole_kib() {
        assert_eq!(whole_kilobytes(1024), 1);
        assert_eq!(whole_kilobytes(4096), 4);
        assert_eq!(whole_kilobytes(5000), 4);
    }
}
