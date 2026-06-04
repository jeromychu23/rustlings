// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Beginner
// Scenario: Backend/API
//
// Task: Apply route commands to a route list.

enum RouteCommand {
    Add(String),
    Remove(String),
    Clear,
}

mod routing {
    use super::RouteCommand;

    pub fn apply(commands: Vec<RouteCommand>) -> Vec<String> {
        let mut routes = Vec::new();

        for command in commands {
            match command {
                RouteCommand::Add(path) => routes.push(path),
                RouteCommand::Remove(path) => {
                    let mut kept = Vec::new();
                    for route in routes {
                        if route != path {
                            kept.push(route);
                        }
                    }
                    routes = kept;
                }
                RouteCommand::Clear => routes.clear(),
            }
        }

        routes
    }
}

fn main() {
    let _ = routing::apply(Vec::new());
}

#[cfg(test)]
mod tests {
    use super::{routing, RouteCommand};

    #[test]
    fn applies_route_commands() {
        let commands = vec![
            RouteCommand::Add("/health".to_string()),
            RouteCommand::Add("/metrics".to_string()),
            RouteCommand::Remove("/health".to_string()),
            RouteCommand::Add("/orders".to_string()),
        ];

        assert_eq!(routing::apply(commands), vec!["/metrics".to_string(), "/orders".to_string()]);
        assert_eq!(routing::apply(vec![RouteCommand::Add("/x".to_string()), RouteCommand::Clear]), Vec::<String>::new());
    }
}
