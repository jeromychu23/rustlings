#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 05 Vecs
// Difficulty: Beginner
// Scenario: Infrastructure
//
// Task: Append a request id to a mutable queue.

fn enqueue_request(queue: &mut Vec<String>, request_id: String) {
    // TODO: Push request_id into queue.
    queue.push(request_id);
}

fn main() {
    let mut queue = Vec::new();
    enqueue_request(&mut queue, "req-1".to_string());
    println!("{queue:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_request() {
        let mut queue = vec!["req-0".to_string()];
        enqueue_request(&mut queue, "req-1".to_string());
        assert_eq!(queue, vec!["req-0".to_string(), "req-1".to_string()]);
    }
}
