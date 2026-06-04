// GPT Rustlings extension
// Topic: 07 Structs
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Mutate a deployment struct by scaling replicas.

#[derive(Debug, PartialEq, Eq)]
struct Deployment {
    service: String,
    version: String,
    replicas: u16,
}

impl Deployment {
    fn scale_to(&mut self, replicas: u16) {
        self.replicas = replicas;
    }
}

fn main() {
    let mut deployment = Deployment { service: "api".to_string(), version: "1.0".to_string(), replicas: 1 };
    deployment.scale_to(3);
    println!("{deployment:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_deployment() {
        let mut deployment = Deployment { service: "api".to_string(), version: "1.0".to_string(), replicas: 1 };
        deployment.scale_to(4);
        assert_eq!(deployment.replicas, 4);
        assert_eq!(deployment.service, "api");
    }
}
