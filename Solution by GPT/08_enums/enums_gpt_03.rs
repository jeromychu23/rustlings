// GPT Rustlings extension
// Topic: 08 Enums
// Difficulty: Intermediate
// Scenario: CLI tooling
//
// Task: Extract the service target from a command enum with payloads.

enum Command {
    Start(String),
    Stop(String),
    Restart(String),
}

fn command_target(command: Command) -> String {
    match command {
        Command::Start(service) => service,
        Command::Stop(service) => service,
        Command::Restart(service) => service,
    }
}

fn main() {
    println!("{}", command_target(Command::Start("api".to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_command_target() {
        assert_eq!(command_target(Command::Start("api".to_string())), "api");
        assert_eq!(command_target(Command::Stop("worker".to_string())), "worker");
        assert_eq!(command_target(Command::Restart("cache".to_string())), "cache");
    }
}
