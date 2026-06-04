#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 09 Strings
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Append a query string to an owned path.

fn append_query(mut path: String, query: &str) -> String {
    // TODO: Append '?' and query to path, then return path.
    path
}

fn main() {
    println!("{}", append_query("/search".to_string(), "q=rust"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_query() {
        assert_eq!(append_query("/search".to_string(), "q=rust"), "/search?q=rust");
        assert_eq!(append_query("/items".to_string(), "page=2"), "/items?page=2");
    }
}
