#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Advanced
// Scenario: In-place buffer normalization
//
// Task: Use `AsMut<[u8]>` to uppercase ASCII bytes without allocating a new Vec.

fn uppercase_ascii_in_place<T: AsMut<[u8]>>(buffer: &mut T) {
    // TODO: Borrow the mutable byte slice and uppercase it in place.
}

fn main() {
    let mut bytes = b"apu-ok".to_vec();
    uppercase_ascii_in_place(&mut bytes);
    println!("{:?}", bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_vec_buffer_in_place() {
        let mut bytes = b"apu-ok".to_vec();
        uppercase_ascii_in_place(&mut bytes);
        assert_eq!(bytes, b"APU-OK");
    }

    #[test]
    fn mutates_array_buffer_in_place() {
        let mut bytes = *b"egt-12";
        uppercase_ascii_in_place(&mut bytes);
        assert_eq!(&bytes, b"EGT-12");
    }
}
