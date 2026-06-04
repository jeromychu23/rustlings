// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Construct a request struct from method and path values.

#[derive(Debug, PartialEq, Eq)]
struct ApiRequest {
    method: String,
    path: String,
}

fn new_request(method: &str, path: &str) -> ApiRequest {
    ApiRequest {
        method: method.to_string(),
        path: path.to_string(),
    }
}

fn main() {
    println!("{:?}", new_request("GET", "/health"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_request() {
        let request = new_request("GET", "/health");
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/health");
    }
}
