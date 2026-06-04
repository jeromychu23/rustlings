#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Map response variants to numeric status codes.

enum Status {
    Ok,
    Created,
    NotFound,
    InternalError,
}

fn status_code(status: Status) -> u16 {
    // TODO: Return 200, 201, 404, or 500.
    0
}

fn main() {
    println!("{}", status_code(Status::Ok));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_status_codes() {
        assert_eq!(status_code(Status::Ok), 200);
        assert_eq!(status_code(Status::Created), 201);
        assert_eq!(status_code(Status::NotFound), 404);
        assert_eq!(status_code(Status::InternalError), 500);
    }
}
