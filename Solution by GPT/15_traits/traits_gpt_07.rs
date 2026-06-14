// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Intermediate
// Scenario: Source comparison
//
// Task: Compare `impl Trait` parameters with a generic type parameter.

trait Source {
    fn system(&self) -> &str;
}

struct ApiSource {
    system: String,
}

struct CsvSource {
    system: String,
}

impl Source for ApiSource {
    fn system(&self) -> &str {
        &self.system
    }
}

impl Source for CsvSource {
    fn system(&self) -> &str {
        &self.system
    }
}

fn same_system(left: impl Source, right: impl Source) -> bool {
    left.system() == right.system()
}

fn same_system_strict<T: Source>(left: T, right: T) -> bool {
    left.system() == right.system()
}

fn main() {
    let api = ApiSource {
        system: String::from("mx"),
    };
    let csv = CsvSource {
        system: String::from("mx"),
    };
    println!("{}", same_system(api, csv));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_trait_accepts_different_concrete_types() {
        let api = ApiSource {
            system: String::from("mx"),
        };
        let csv = CsvSource {
            system: String::from("mx"),
        };

        assert!(same_system(api, csv));
    }

    #[test]
    fn generic_type_parameter_requires_same_concrete_type() {
        let left = ApiSource {
            system: String::from("mx"),
        };
        let right = ApiSource {
            system: String::from("finance"),
        };

        assert!(!same_system_strict(left, right));
    }
}
