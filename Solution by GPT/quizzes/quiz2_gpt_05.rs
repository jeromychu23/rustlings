// GPT Rustlings extension
// Topic: Quiz 2 Review
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Process queue commands that move and rename owned strings.

enum QueueCommand {
    Push(String),
    Pop,
    Rename { from: String, to: String },
}

mod queue {
    use super::QueueCommand;

    pub fn process(initial: Vec<String>, commands: Vec<QueueCommand>) -> Vec<String> {
        let mut queue = initial;

        for command in commands {
            match command {
                QueueCommand::Push(item) => queue.push(item),
                QueueCommand::Pop => {
                    if !queue.is_empty() {
                        queue.remove(0);
                    }
                }
                QueueCommand::Rename { from, to } => {
                    for item in &mut queue {
                        if *item == from {
                            *item = to.clone();
                        }
                    }
                }
            }
        }

        queue
    }
}

fn main() {
    let _ = queue::process(Vec::new(), Vec::new());
}

#[cfg(test)]
mod tests {
    use super::{queue, QueueCommand};

    #[test]
    fn processes_queue_commands() {
        let initial = vec!["job-a".to_string()];
        let commands = vec![
            QueueCommand::Push("job-b".to_string()),
            QueueCommand::Rename { from: "job-a".to_string(), to: "job-a-retry".to_string() },
            QueueCommand::Pop,
        ];

        assert_eq!(queue::process(initial, commands), vec!["job-b".to_string()]);
    }
}
