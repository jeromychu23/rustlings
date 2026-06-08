#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Intermediate
// Scenario: Networking
//
// Task: Decide if a connection state can send data.

enum ConnectionState {
    New,
    Open,
    Draining,
    Closed,
}

fn can_send(state: ConnectionState) -> bool {
    // TODO: Only Open and Draining can send.
    matches!(state, ConnectionState::Open | ConnectionState::Draining)
}

fn main() {
    println!("{}", can_send(ConnectionState::Open));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_send_capability() {
        assert!(!can_send(ConnectionState::New));
        assert!(can_send(ConnectionState::Open));
        assert!(can_send(ConnectionState::Draining));
        assert!(!can_send(ConnectionState::Closed));
    }
}
