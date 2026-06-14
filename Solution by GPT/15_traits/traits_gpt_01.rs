// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Beginner
// Scenario: Text normalization
//
// Task: Implement a trait for String.

trait AddAuditSuffix {
    fn add_audit_suffix(self) -> Self;
}

impl AddAuditSuffix for String {
    fn add_audit_suffix(self) -> Self {
        self + "-AUDIT"
    }
}

fn main() {
    println!("{}", String::from("WO-1").add_audit_suffix());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_suffix_once() {
        assert_eq!(String::from("WO-1").add_audit_suffix(), "WO-1-AUDIT");
    }

    #[test]
    fn appends_suffix_twice() {
        assert_eq!(
            String::from("WO-1").add_audit_suffix().add_audit_suffix(),
            "WO-1-AUDIT-AUDIT"
        );
    }
}
