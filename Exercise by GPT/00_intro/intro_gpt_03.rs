#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 00 Intro
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Format a simple deployment note.

fn release_note(service: &str, version: &str, build: u32) -> String {
    // TODO: Return: "<service>@<version> build <build>"
    format!("{service}@{version} build {build}")
}

fn main() {
    println!("{}", release_note("search", "1.4.2", 17));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_release_note() {
        assert_eq!(release_note("search", "1.4.2", 17), "search@1.4.2 build 17");
        assert_eq!(release_note("api", "0.9.0", 3), "api@0.9.0 build 3");
    }
}
