// GPT Rustlings extension
// Topic: 10 Modules
// Difficulty: Advanced
// Scenario: Security
//
// Task: Use a public enum with a nested permission-check module.

mod auth {
    pub enum Role {
        Viewer,
        Operator,
        Admin,
    }

    pub mod checks {
        use super::Role;

        pub fn can_deploy(role: Role) -> bool {
            match role {
                Role::Viewer => false,
                Role::Operator | Role::Admin => true,
            }
        }
    }
}

fn main() {
    println!("{}", auth::checks::can_deploy(auth::Role::Admin));
}

#[cfg(test)]
mod tests {
    use super::auth::{checks, Role};

    #[test]
    fn checks_deploy_permission() {
        assert!(!checks::can_deploy(Role::Viewer));
        assert!(checks::can_deploy(Role::Operator));
        assert!(checks::can_deploy(Role::Admin));
    }
}
