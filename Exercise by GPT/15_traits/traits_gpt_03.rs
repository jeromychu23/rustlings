#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Beginner
// Scenario: Licensing
//
// Task: Add the correct default method implementation.

trait Licensed {
    fn licensing_info(&self) -> String {
        // TODO: Return "Default license".
        String::new()
    }
}

struct InternalTool;
struct VendorTool;

impl Licensed for InternalTool {}
impl Licensed for VendorTool {}

fn main() {
    println!("{}", InternalTool.licensing_info());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_types_use_default_license() {
        assert_eq!(InternalTool.licensing_info(), "Default license");
        assert_eq!(VendorTool.licensing_info(), "Default license");
    }
}
