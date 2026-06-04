#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Interpret a metric sample tuple.

fn metric_label(sample: (&str, u32, bool)) -> String {
    // TODO: Return "<name>=<value> active=<flag>".
    String::new()
}

fn main() {
    println!("{}", metric_label(("requests", 42, true)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_metric_label() {
        assert_eq!(metric_label(("requests", 42, true)), "requests=42 active=true");
        assert_eq!(metric_label(("errors", 0, false)), "errors=0 active=false");
    }
}
