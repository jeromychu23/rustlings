#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 14 Generics
// Difficulty: Advanced
// Scenario: Batch processing
//
// Task: Implement a generic batch container.

struct Batch<T> {
    source: String,
    items: Vec<T>,
}

impl<T> Batch<T> {
    fn new(source: &str) -> Self {
        // TODO: Store the source and start with an empty item list.
        Batch {
            source: String::new(),
            items: Vec::new(),
        }
    }

    fn source(&self) -> &str {
        // TODO: Return the source.
        ""
    }

    fn push(&mut self, item: T) {
        // TODO: Append the item.
    }

    fn len(&self) -> usize {
        // TODO: Return the number of items.
        0
    }

    fn is_empty(&self) -> bool {
        // TODO: Return whether there are no items.
        true
    }

    fn into_items(self) -> Vec<T> {
        // TODO: Consume the batch and return the items.
        Vec::new()
    }

    fn map<U, F>(self, mut f: F) -> Batch<U>
    where
        F: FnMut(T) -> U,
    {
        // TODO: Preserve the source and transform every item.
        Batch {
            source: self.source,
            items: Vec::new(),
        }
    }
}

fn main() {
    let batch: Batch<u32> = Batch::new("manual");
    println!("{} {}", batch.source(), batch.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_source_and_items() {
        let mut batch = Batch::new("foundry_transform");
        assert_eq!(batch.source(), "foundry_transform");
        assert!(batch.is_empty());

        batch.push("WO-1".to_string());
        batch.push("WO-2".to_string());

        assert_eq!(batch.len(), 2);
        assert!(!batch.is_empty());
        assert_eq!(
            batch.into_items(),
            vec!["WO-1".to_string(), "WO-2".to_string()]
        );
    }

    #[test]
    fn maps_items_to_a_new_type() {
        let mut batch = Batch::new("hours");
        batch.push(4_u32);
        batch.push(9_u32);

        let labels = batch.map(|hours| format!("{hours}h"));

        assert_eq!(labels.source(), "hours");
        assert_eq!(
            labels.into_items(),
            vec!["4h".to_string(), "9h".to_string()]
        );
    }
}
