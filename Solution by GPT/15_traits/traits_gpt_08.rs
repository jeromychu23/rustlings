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
        if self.id.is_empty() {
            Err(String::from("work order id is required"))
        } else if self.estimated_hours == 0 {
            Err(String::from("estimated hours must be greater than 0"))
        } else {
            Ok(())
        }
    }
}

impl Validate for FlightLeg {
    fn validate(&self) -> Result<(), String> {
        if self.flight_no.is_empty() {
            Err(String::from("flight number is required"))
        } else if self.origin.is_empty() || self.destination.is_empty() {
            Err(String::from("origin and destination are required"))
        } else if self.origin == self.destination {
            Err(String::from("origin and destination must be different"))
        } else {
            Ok(())
        }
    }
}

fn collect_invalid<T: Validate>(items: &[T]) -> Vec<String> {
    let mut errors = Vec::new();

    for item in items {
        if let Err(error) = item.validate() {
            errors.push(error);
        }
    }

    errors
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
