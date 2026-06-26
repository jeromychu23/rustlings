// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Beginner
// Scenario: Byte length for text payloads
//
// Task: Accept String or &str and return UTF-8 byte length.

fn byte_len<T: AsRef<str>>(input: T) -> usize {
    input.as_ref().len()
}

fn main() {
    println!("{}", byte_len("Café"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_bytes_equal_characters() {
        assert_eq!(byte_len("Cafe"), 4);
    }

    #[test]
    fn utf8_bytes_can_exceed_character_count() {
        assert_eq!(byte_len("Café"), 5);
    }

    #[test]
    fn accepts_owned_string() {
        assert_eq!(byte_len(String::from("A320")), 4);
    }
}
