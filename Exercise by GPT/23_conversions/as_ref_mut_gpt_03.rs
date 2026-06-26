#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Intermediate
// Scenario: Generic tail number API
//
// Task: Accept String, &String, or &str without forcing callers to allocate.

fn normalize_tail_number<T: AsRef<str>>(tail_number: T) -> String {
    // TODO: Borrow as str, trim, and uppercase.
    tail_number.as_ref().to_string()
}

fn main() {
    println!("{}", normalize_tail_number(" b-1234 "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_str_literal() {
        assert_eq!(normalize_tail_number(" b-1234 "), "B-1234");
    }

    #[test]
    fn accepts_owned_string() {
        assert_eq!(normalize_tail_number(String::from(" b-5678 ")), "B-5678");
    }

    #[test]
    fn can_borrow_string_without_moving_it() {
        let tail = String::from(" b-9999 ");
        assert_eq!(normalize_tail_number(&tail), "B-9999");
        assert_eq!(tail, " b-9999 ");
    }
}
