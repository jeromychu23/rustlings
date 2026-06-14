// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Intermediate
// Scenario: Pipeline records
//
// Task: Store any payload type in a record and transform the payload.

struct Record<T> {
    id: String,
    payload: T,
}

impl<T> Record<T> {
    fn new(id: &str, payload: T) -> Self {
        Record {
            id: id.to_string(),
            payload,
        }
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn payload(&self) -> &T {
        &self.payload
    }

    fn map_payload<U, F>(self, f: F) -> Record<U>
    where
        F: FnOnce(T) -> U,
    {
        Record {
            id: self.id,
            payload: f(self.payload),
        }
    }
}

fn main() {
    let record = Record::new("WO-1", 4);
    println!("{} {:?}", record.id(), record.payload());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_payload_and_id() {
        let record = Record::new("WO-100", String::from("open"));
        assert_eq!(record.id(), "WO-100");
        assert_eq!(record.payload(), "open");
    }

    #[test]
    fn maps_payload_without_losing_id() {
        let record = Record::new("WO-200", 7);
        let record = record.map_payload(|hours| format!("{hours}h"));

        assert_eq!(record.id(), "WO-200");
        assert_eq!(record.payload(), "7h");
    }
}
