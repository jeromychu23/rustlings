#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 23 Conversions - from_into
// Difficulty: Advanced
// Scenario: Raw defect DTO to domain event
//
// Task: Chain value conversions across DTO, normalized domain, and event types.

#[derive(Debug)]
struct ExternalDefectDto {
    tail_number: String,
    ata_code: String,
    description: String,
}

#[derive(Debug, PartialEq, Eq)]
struct NormalizedDefect {
    tail_number: String,
    ata_code: String,
    description: String,
}

#[derive(Debug, PartialEq, Eq)]
struct DefectEvent {
    key: String,
    description: String,
}

impl From<ExternalDefectDto> for NormalizedDefect {
    fn from(dto: ExternalDefectDto) -> Self {
        // TODO: Normalize tail_number and ata_code; trim description.
        Self {
            tail_number: String::new(),
            ata_code: String::new(),
            description: String::new(),
        }
    }
}

impl From<NormalizedDefect> for DefectEvent {
    fn from(defect: NormalizedDefect) -> Self {
        // TODO: Build key as "TAIL/ATA".
        Self {
            key: String::new(),
            description: defect.description,
        }
    }
}

fn ingest_defect(dto: ExternalDefectDto) -> DefectEvent {
    // TODO: Convert dto -> NormalizedDefect -> DefectEvent.
    DefectEvent {
        key: String::new(),
        description: String::new(),
    }
}

fn main() {
    let event = ingest_defect(ExternalDefectDto {
        tail_number: " b-1234 ".to_string(),
        ata_code: " 32 ".to_string(),
        description: " tire worn ".to_string(),
    });
    println!("{event:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_dto_before_building_event() {
        let event = ingest_defect(ExternalDefectDto {
            tail_number: " b-1234 ".to_string(),
            ata_code: " 32 ".to_string(),
            description: " tire worn ".to_string(),
        });

        assert_eq!(
            event,
            DefectEvent {
                key: "B-1234/32".to_string(),
                description: "tire worn".to_string(),
            }
        );
    }
}
