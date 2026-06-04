// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Identify HTTP methods that usually write server state.

enum Method {
    Get,
    Post,
    Put,
    Delete,
}

fn is_write_method(method: Method) -> bool {
    match method {
        Method::Get => false,
        Method::Post | Method::Put | Method::Delete => true,
    }
}

fn main() {
    println!("{}", is_write_method(Method::Post));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_write_methods() {
        assert!(!is_write_method(Method::Get));
        assert!(is_write_method(Method::Post));
        assert!(is_write_method(Method::Put));
        assert!(is_write_method(Method::Delete));
    }
}
