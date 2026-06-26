// GPT Rustlings extension
// Topic: 23 Conversions - as_ref_mut
// Difficulty: Intermediate
// Scenario: Generic file path API
//
// Task: Accept PathBuf or &Path and return an uppercase file-stem label.

use std::path::Path;

fn file_stem_label<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_ascii_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn main() {
    println!("{}", file_stem_label(Path::new("/tmp/daily.csv")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_path_reference() {
        assert_eq!(file_stem_label(Path::new("/tmp/daily_inspection.csv")), "DAILY_INSPECTION");
    }

    #[test]
    fn accepts_owned_path_buf() {
        let path = std::path::PathBuf::from("/tmp/fh_report.csv");
        assert_eq!(file_stem_label(path), "FH_REPORT");
    }
}
