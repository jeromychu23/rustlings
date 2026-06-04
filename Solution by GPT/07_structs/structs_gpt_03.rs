// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Implement method syntax for a response struct.

struct ApiResponse {
    status: u16,
    body: String,
}

impl ApiResponse {
    fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

fn main() {
    let response = ApiResponse { status: 200, body: "ok".to_string() };
    println!("{}", response.is_success());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_success() {
        let ok = ApiResponse { status: 204, body: String::new() };
        let fail = ApiResponse { status: 500, body: "error".to_string() };
        assert!(ok.is_success());
        assert!(!fail.is_success());
    }
}
