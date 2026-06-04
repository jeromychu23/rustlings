#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Mutably borrow a log line and append a trace id.

fn append_trace_id(line: &mut String, trace_id: &str) {
    // TODO: Append " trace=<trace_id>" to line.
}

fn main() {
    let mut line = "GET /health".to_string();
    append_trace_id(&mut line, "abc");
    println!("{line}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_trace_id() {
        let mut line = "GET /health".to_string();
        append_trace_id(&mut line, "abc");
        assert_eq!(line, "GET /health trace=abc");
    }
}
