// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Beginner
// Scenario: Character count for labels
//
// Task: Accept String or &str and count Unicode scalar values with `.chars()`.

fn char_count<T: AsRef<str>>(input: T) -> usize {
    input.as_ref().chars().count()
}

fn main() {
    println!("{}", char_count("Café"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_bytes_equal_characters() {
        assert_eq!(char_count("Cafe"), 4);
    }

    #[test]
    fn utf8_character_count_differs_from_bytes() {
        assert_eq!(char_count("Café"), 4);
    }

    #[test]
    fn accepts_owned_string() {
        assert_eq!(char_count(String::from("油壓")), 2);
    }
}
