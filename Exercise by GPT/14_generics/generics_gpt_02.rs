#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Beginner
// Scenario: Metrics
//
// Task: Make `Metric` generic so it can store different value types.

struct Metric<T> {
    name: String,
    value: T,
}

impl<T> Metric<T> {
    fn new(name: &str, value: T) -> Self {
        // TODO: Store the provided metric name.
        Metric {
            name: name.to_string(),
            value,
        }
    }

    fn name(&self) -> &str {
        // TODO: Return the stored name.
        &self.name
    }

    fn into_value(self) -> T {
        // TODO: Return the wrapped value.
        self.value
    }
}

fn main() {
    let metric = Metric::new("open_work_orders", 42);
    println!("{}", metric.name());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_numeric_metric() {
        let metric = Metric::new("open_work_orders", 42_u32);
        assert_eq!(metric.name(), "open_work_orders");
        assert_eq!(metric.into_value(), 42);
    }

    #[test]
    fn stores_text_metric() {
        let metric = Metric::new("aircraft_status", String::from("available"));
        assert_eq!(metric.name(), "aircraft_status");
        assert_eq!(metric.into_value(), "available");
    }
}
