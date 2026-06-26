// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Intermediate
// Scenario: Technician CSV-like field with default fallback
//
// Task: Implement `From<&str>` for "name,level".

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
        let mut parts = value.split(',');
        let (Some(name), Some(level), None) = (parts.next(), parts.next(), parts.next()) else {
            return Self::default();
        };

        let name = name.trim();
        if name.is_empty() {
            return Self::default();
        }

        let Ok(level) = level.trim().parse::<u8>() else {
            return Self::default();
        };

        Self {
            name: name.to_string(),
            level,
        }
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
