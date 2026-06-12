#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: Quiz 1 Review
// Difficulty: Advanced
// Scenario: Infrastructure
//
// Task: Decide admission control from CPU usage and queue depth.

fn admission_decision(cpu_percent: u8, queue_depth: u32) -> &'static str {
    // TODO:
    // - cpu >= 90 or queue_depth > 1000 => "reject"
    // - cpu >= 75 or queue_depth > 500 => "throttle"
    // - otherwise => "accept"
    match (cpu_percent, queue_depth) {
        (cpu, queue) if cpu >= 90 || queue > 1000 => "reject",
        (cpu, queue) if cpu >= 75 || queue > 500 => "throttle",
        _ => "accept",
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
