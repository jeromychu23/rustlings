// GPT Rustlings extension
// Topic: 11 HashMaps
// Difficulty: Intermediate
// Scenario: Infrastructure
//
// Task: Merge incoming metric counts into an existing map.

use std::collections::HashMap;

fn merge_metrics(base: &mut HashMap<String, u32>, incoming: HashMap<String, u32>) {
    for (name, value) in incoming {
        let current = base.entry(name).or_insert(0);
        *current += value;
    }
}

fn main() {
    let mut base = HashMap::new();
    merge_metrics(&mut base, HashMap::new());
    println!("{base:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_metrics() {
        let mut base = HashMap::from([
            ("requests".to_string(), 10),
            ("errors".to_string(), 1),
        ]);
        let incoming = HashMap::from([
            ("requests".to_string(), 5),
            ("timeouts".to_string(), 2),
        ]);

        merge_metrics(&mut base, incoming);

        assert_eq!(base.get("requests"), Some(&15));
        assert_eq!(base.get("errors"), Some(&1));
        assert_eq!(base.get("timeouts"), Some(&2));
    }
}
