#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Intermediate
// Scenario: Fallback data
//
// Task: Return the primary value when present, otherwise return the fallback.

fn first_present<T>(primary: Option<T>, fallback: Option<T>) -> Option<T> {
    // TODO: Return `primary` when it is `Some`, otherwise return `fallback`.
    None
}

fn main() {
    println!("{:?}", first_present(Some(1), Some(2)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_primary_when_present() {
        assert_eq!(first_present(Some(10), Some(20)), Some(10));
    }

    #[test]
    fn chooses_fallback_when_primary_is_missing() {
        assert_eq!(
            first_present(None, Some(String::from("backup"))),
            Some("backup".to_string())
        );
    }

    #[test]
    fn returns_none_when_both_are_missing() {
        let value: Option<u32> = first_present(None, None);
        assert_eq!(value, None);
    }
}
