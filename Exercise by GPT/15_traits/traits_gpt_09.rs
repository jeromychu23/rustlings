#![allow(unused_variables, unused_mut)]

// GPT Rustlings extension
// Topic: 15 Traits
// Difficulty: Advanced
// Scenario: Maintenance severity
//
// Task: Use default trait methods and override behavior where needed.

trait SeverityRule {
    fn base_severity(&self) -> u8;

    fn severity(&self) -> u8 {
        // TODO: The default severity should be the base severity.
        self.base_severity()
    }

    fn is_high_priority(&self) -> bool {
        // TODO: High priority means severity is at least 8.
        self.severity() >= 8
    }
}

struct DelayEvent {
    delayed_minutes: u32,
}

struct AogEvent {
    blocked_hours: u32,
}

struct RoutineInspection;

impl SeverityRule for DelayEvent {
    fn base_severity(&self) -> u8 {
        // TODO: 0..=29 minutes => 3, 30..=119 => 6, 120+ => 8.
        match self.delayed_minutes {
            (0..=29) => 3,
            (30..=119) => 6,
            (120..) => 8,
        }
    }
}

impl SeverityRule for AogEvent {
    fn base_severity(&self) -> u8 {
        // TODO: Base severity for AOG is 9.
        9
    }

    fn severity(&self) -> u8 {
        // TODO: Override so blocked_hours >= 24 returns 10, otherwise base severity.
        if self.blocked_hours >= 24 {
            10
        } else {
            self.base_severity()
        }
    }
}

impl SeverityRule for RoutineInspection {
    fn base_severity(&self) -> u8 {
        // TODO: Routine inspection severity is 2.
        2
    }
}

fn main() {
    println!(
        "{}",
        DelayEvent {
            delayed_minutes: 45
        }
        .severity()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_delay_severity() {
        assert_eq!(
            DelayEvent {
                delayed_minutes: 10
            }
            .severity(),
            3
        );
        assert_eq!(
            DelayEvent {
                delayed_minutes: 45
            }
            .severity(),
            6
        );
        assert_eq!(
            DelayEvent {
                delayed_minutes: 150
            }
            .severity(),
            8
        );
    }

    #[test]
    fn overrides_aog_severity() {
        assert_eq!(AogEvent { blocked_hours: 2 }.severity(), 9);
        assert_eq!(AogEvent { blocked_hours: 24 }.severity(), 10);
        assert!(AogEvent { blocked_hours: 24 }.is_high_priority());
    }

    #[test]
    fn uses_default_for_routine_inspection() {
        let inspection = RoutineInspection;
        assert_eq!(inspection.severity(), 2);
        assert!(!inspection.is_high_priority());
    }
}
