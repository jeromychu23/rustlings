// GPT Rustlings extension
// Topic: 04 Primitive Types
// Difficulty: Advanced
// Scenario: Backend/API
//
// Task: Interpret a metric sample tuple.

fn metric_label(sample: (&str, u32, bool)) -> String {
    let (name, value, active) = sample;
    format!("{name}={value} active={active}")
}

fn main() {
    println!("{}", metric_label(("requests", 42, true)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_metric_label() {
        assert_eq!(metric_label(("requests", 42, true)), "requests=42 active=true");
        assert_eq!(metric_label(("errors", 0, false)), "errors=0 active=false");
    }
}
