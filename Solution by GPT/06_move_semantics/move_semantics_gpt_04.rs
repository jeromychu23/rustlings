// GPT Rustlings extension
// Topic: 06 Move Semantics
// Difficulty: Intermediate
// Scenario: Security
//
// Task: Clone a payload for audit while keeping the original borrowed.

fn clone_for_audit(payload: &String) -> (String, usize) {
    (payload.clone(), payload.len())
}

fn main() {
    println!("{:?}", clone_for_audit(&"secret".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_for_audit() {
        let payload = "token".to_string();
        let (copy, len) = clone_for_audit(&payload);
        assert_eq!(copy, "token");
        assert_eq!(len, 5);
        assert_eq!(payload, "token");
    }
}
