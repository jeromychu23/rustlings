#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Advanced
// Scenario: Validation
//
// Task: Implement validation for two record types and collect invalid reasons.

trait Validate {
    fn validate(&self) -> Result<(), String>;
}

struct WorkOrder {
    id: String,
    estimated_hours: u32,
}

struct FlightLeg {
    flight_no: String,
    origin: String,
    destination: String,
}

impl Validate for WorkOrder {
    fn validate(&self) -> Result<(), String> {
        // TODO: id must not be empty and estimated_hours must be greater than 0.
        if self.id.is_empty() {
            Err("work order id is required".to_string())
        } else if self.estimated_hours == 0 {
            Err("estimated hours must be greater than 0".to_string())
        } else {
            Ok(())
        }
    }
}

impl Validate for FlightLeg {
    fn validate(&self) -> Result<(), String> {
        // TODO: flight_no, origin, and destination must be non-empty.
        // TODO: origin and destination must be different.
        if self.flight_no.is_empty() {
            Err("flight number is required".to_string())
        } else if self.origin.is_empty() {
            Err("origin is required".to_string())
        } else if self.destination.is_empty() {
            Err("destination is required".to_string())
        } else if self.origin == self.destination {
            Err("origin and destination must be different".to_string())
        } else {
            Ok(())
        }
    }
}

fn collect_invalid<T: Validate>(items: &[T]) -> Vec<String> {
    // TODO: Return every validation error message.
    let mut output = Vec::new();
    for item in items {
        if let Err(error) = item.validate() {
            output.push(error)
        }
    }
    output
}

fn main() {
    let orders = vec![WorkOrder {
        id: String::from("WO-1"),
        estimated_hours: 4,
    }];
    println!("{:?}", collect_invalid(&orders));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_work_orders() {
        let orders = vec![
            WorkOrder {
                id: String::from("WO-1"),
                estimated_hours: 4,
            },
            WorkOrder {
                id: String::new(),
                estimated_hours: 3,
            },
            WorkOrder {
                id: String::from("WO-2"),
                estimated_hours: 0,
            },
        ];

        assert_eq!(
            collect_invalid(&orders),
            vec![
                "work order id is required".to_string(),
                "estimated hours must be greater than 0".to_string()
            ]
        );
    }

    #[test]
    fn validates_flight_legs() {
        let legs = vec![
            FlightLeg {
                flight_no: String::from("CI001"),
                origin: String::from("TPE"),
                destination: String::from("LAX"),
            },
            FlightLeg {
                flight_no: String::new(),
                origin: String::from("TPE"),
                destination: String::from("LAX"),
            },
            FlightLeg {
                flight_no: String::from("CI002"),
                origin: String::from("TPE"),
                destination: String::from("TPE"),
            },
        ];

        assert_eq!(
            collect_invalid(&legs),
            vec![
                "flight number is required".to_string(),
                "origin and destination must be different".to_string()
            ]
        );
    }
}
