#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Intermediate
// Scenario: Backend/API
//
// Task: Call a function inside a nested API version module.

mod api {
    pub mod v1 {
        pub fn prefixed(path: &str) -> String {
            // TODO: Return "/v1<path>".
            String::new()
        }
    }
}

fn main() {
    println!("{}", api::v1::prefixed("/users"));
}

#[cfg(test)]
mod tests {
    use super::api;

    #[test]
    fn prefixes_v1_routes() {
        assert_eq!(api::v1::prefixed("/users"), "/v1/users");
        assert_eq!(api::v1::prefixed("/orders"), "/v1/orders");
    }
}
