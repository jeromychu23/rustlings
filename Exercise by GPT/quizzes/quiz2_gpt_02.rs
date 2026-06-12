#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Apply route commands to a route list.

#[derive(Debug)]
enum RouteCommand {
    Add(String),
    Remove(String),
    Clear,
}

mod routing {
    use super::RouteCommand;

    pub fn apply(commands: Vec<RouteCommand>) -> Vec<String> {
        // TODO: Apply Add, Remove, and Clear in order.
        let mut routes = Vec::new();
        for c in commands {
            match c {
                RouteCommand::Add(route) => routes.push(route),
                RouteCommand::Remove(route) => routes.retain(|r| r != &route),
                RouteCommand::Clear => routes.clear(),
            }
        }
        routes
    }
}

fn main() {
    let commands = vec![
        RouteCommand::Add("/health".to_string()),
        RouteCommand::Add("/metrics".to_string()),
        RouteCommand::Remove("/health".to_string()),
        RouteCommand::Add("/orders".to_string()),
    ];
    let new_cmd = routing::apply(commands);
    println!("{:?}", new_cmd)
}

#[cfg(test)]
mod tests {
    use super::{RouteCommand, routing};

    #[test]
    fn applies_route_commands() {
        let commands = vec![
            RouteCommand::Add("/health".to_string()),
            RouteCommand::Add("/metrics".to_string()),
            RouteCommand::Remove("/health".to_string()),
            RouteCommand::Add("/orders".to_string()),
        ];

        assert_eq!(
            routing::apply(commands),
            vec!["/metrics".to_string(), "/orders".to_string()]
        );
        assert_eq!(
            routing::apply(vec![
                RouteCommand::Add("/x".to_string()),
                RouteCommand::Clear
            ]),
            Vec::<String>::new()
        );
    }
}
