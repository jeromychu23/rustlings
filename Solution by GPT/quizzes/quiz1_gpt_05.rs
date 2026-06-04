// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Decide admission control from CPU usage and queue depth.

fn admission_decision(cpu_percent: u8, queue_depth: u32) -> &'static str {
    if cpu_percent >= 90 || queue_depth > 1000 {
        "reject"
    } else if cpu_percent >= 75 || queue_depth > 500 {
        "throttle"
    } else {
        "accept"
    }
}

fn main() {
    println!("{}", admission_decision(80, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decides_admission() {
        assert_eq!(admission_decision(95, 0), "reject");
        assert_eq!(admission_decision(70, 1200), "reject");
        assert_eq!(admission_decision(80, 100), "throttle");
        assert_eq!(admission_decision(20, 10), "accept");
    }
}
