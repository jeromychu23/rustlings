#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Advanced
// Scenario: Mutable cycle counter
//
// Task: Use `AsMut<u32>` to mutate a counter inside a wrapper such as Box<u32>.

struct CycleCounter(u32);

impl AsMut<u32> for CycleCounter {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

fn add_cycles<T: AsMut<u32>>(counter: &mut T, delta: u32) {
    // TODO: Borrow the inner u32 with `as_mut()` and add delta in place.
}

fn main() {
    let mut counter = Box::new(10_u32);
    add_cycles(&mut counter, 5);
    println!("{}", counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_boxed_counter() {
        let mut counter = Box::new(10_u32);
        add_cycles(&mut counter, 5);
        assert_eq!(*counter, 15);
    }

    #[test]
    fn mutates_custom_counter_wrapper() {
        let mut counter = CycleCounter(20);

        add_cycles(&mut counter, 7);

        assert_eq!(counter.0, 27);
    }
}
