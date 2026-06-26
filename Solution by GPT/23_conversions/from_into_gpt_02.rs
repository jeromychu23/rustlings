// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Beginner
// Scenario: Raw aircraft record normalization
//
// Task: Convert a raw owned record into a normalized domain record.

#[derive(Debug)]
struct RawAircraftRecord {
    tail_number: String,
    model: String,
}

#[derive(Debug, PartialEq, Eq)]
struct AircraftRecord {
    tail_number: String,
    model: String,
}

impl From<RawAircraftRecord> for AircraftRecord {
    fn from(raw: RawAircraftRecord) -> Self {
        Self {
            tail_number: raw.tail_number.trim().to_ascii_uppercase(),
            model: raw.model.trim().to_ascii_uppercase(),
        }
    }
}

fn main() {
    let raw = RawAircraftRecord {
        tail_number: " b-1234 ".to_string(),
        model: " a320 ".to_string(),
    };
    let record: AircraftRecord = raw.into();
    println!("{record:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_raw_aircraft_record() {
        let raw = RawAircraftRecord {
            tail_number: " b-1234 ".to_string(),
            model: " a320 ".to_string(),
        };

        assert_eq!(
            AircraftRecord::from(raw),
            AircraftRecord {
                tail_number: "B-1234".to_string(),
                model: "A320".to_string(),
            }
        );
    }
}
