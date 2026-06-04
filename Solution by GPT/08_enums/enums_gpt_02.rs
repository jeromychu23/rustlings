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
    match status {
        Status::Ok => 200,
        Status::Created => 201,
        Status::NotFound => 404,
        Status::InternalError => 500,
    }
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
