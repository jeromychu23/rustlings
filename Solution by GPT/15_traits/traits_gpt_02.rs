// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Beginner
// Scenario: Queue tagging
//
// Task: Implement a consuming trait method for Vec<String>.

trait AppendReviewStep {
    fn append_review_step(self) -> Self;
}

impl AppendReviewStep for Vec<String> {
    fn append_review_step(mut self) -> Self {
        self.push(String::from("review"));
        self
    }
}

fn main() {
    println!("{:?}", vec![String::from("ingest")].append_review_step());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_review_step_to_pipeline() {
        let steps = vec![String::from("ingest"), String::from("validate")].append_review_step();
        assert_eq!(
            steps,
            vec![
                String::from("ingest"),
                String::from("validate"),
                String::from("review")
            ]
        );
    }
}
