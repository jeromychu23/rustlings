#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Intermediate
// Scenario: Technician CSV-like field with default fallback
//
// Task: Implement `From<&str>` for "name,level".
// This is intentionally infallible, so invalid input falls back to Default.

#[derive(Debug, PartialEq, Eq)]
struct Technician {
    name: String,
    level: u8,
}

impl Default for Technician {
    fn default() -> Self {
        Self {
            name: "UNKNOWN".to_string(),
            level: 0,
        }
    }
}

impl From<&str> for Technician {
    fn from(value: &str) -> Self {
        // TODO: Parse "name,level"; return Technician::default() for invalid input.
        Self::default()
    }
}

fn main() {
    println!("{:?}", Technician::from("Ada,3"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_technician() {
        assert_eq!(
            Technician::from("Ada,3"),
            Technician {
                name: "Ada".to_string(),
                level: 3,
            }
        );
    }

    #[test]
    fn invalid_shape_uses_default() {
        assert_eq!(Technician::from("Ada"), Technician::default());
        assert_eq!(Technician::from("Ada,3,extra"), Technician::default());
    }

    #[test]
    fn invalid_value_uses_default() {
        assert_eq!(Technician::from(",3"), Technician::default());
        assert_eq!(Technician::from("Ada,senior"), Technician::default());
    }
}
