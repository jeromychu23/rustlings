#![allow(unused_variables)]

// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Advanced
// Scenario: Recurring maintenance schedule
//
// Task:
// 1. Validate that the interval is non-zero.
// 2. Implement Iterator<Item = DueEvent> for a finite recurring schedule.
// 3. Use checked arithmetic so overflow ends the iterator safely.
// 4. Build labels for events at or below `max_hours` with `take_while()` and `map()`.
//
// Do not use loops, recursion, manual index traversal, or an intermediate Vec.

#[derive(Debug, PartialEq, Eq)]
enum ScheduleError {
    ZeroInterval,
}

#[derive(Debug, PartialEq, Eq)]
struct DueEvent {
    sequence: usize,
    due_at_hours: u32,
}

struct MaintenanceSchedule {
    next_due: Option<u32>,
    interval_hours: u32,
    remaining: usize,
    next_sequence: usize,
}

impl MaintenanceSchedule {
    fn new(first_due_hours: u32, interval_hours: u32, count: usize) -> Result<Self, ScheduleError> {
        // TODO: Reject a zero interval and initialize the iterator state.
        Err(ScheduleError::ZeroInterval)
    }
}

impl Iterator for MaintenanceSchedule {
    type Item = DueEvent;

    // TODO: Match the Iterator trait's required return type, then yield the
    // current event and advance the state with checked_add().
    fn next(&mut self) -> Self::Item {
        DueEvent {
            sequence: 0,
            due_at_hours: 0,
        }
    }
}

fn due_labels_within(schedule: MaintenanceSchedule, max_hours: u32) -> Vec<String> {
    // TODO: Use take_while(), map(), and collect().
    Vec::new()
}

fn main() {
    let schedule = MaintenanceSchedule::new(100, 50, 4).unwrap();
    println!("{:?}", due_labels_within(schedule, 220));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_a_finite_schedule_in_order() {
        let events: Vec<_> = MaintenanceSchedule::new(100, 50, 3).unwrap().collect();

        assert_eq!(
            events,
            [
                DueEvent {
                    sequence: 1,
                    due_at_hours: 100,
                },
                DueEvent {
                    sequence: 2,
                    due_at_hours: 150,
                },
                DueEvent {
                    sequence: 3,
                    due_at_hours: 200,
                },
            ]
        );
    }

    #[test]
    fn rejects_a_zero_interval() {
        assert!(matches!(
            MaintenanceSchedule::new(100, 0, 3),
            Err(ScheduleError::ZeroInterval)
        ));
    }

    #[test]
    fn count_zero_is_immediately_exhausted() {
        let mut schedule = MaintenanceSchedule::new(100, 50, 0).unwrap();
        assert_eq!(schedule.next(), None);
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn remains_exhausted_after_the_requested_count() {
        let mut schedule = MaintenanceSchedule::new(100, 50, 1).unwrap();
        assert_eq!(
            schedule.next(),
            Some(DueEvent {
                sequence: 1,
                due_at_hours: 100,
            })
        );
        assert_eq!(schedule.next(), None);
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn overflow_ends_the_iterator_safely() {
        let mut schedule = MaintenanceSchedule::new(u32::MAX - 5, 10, 3).unwrap();
        assert_eq!(
            schedule.next(),
            Some(DueEvent {
                sequence: 1,
                due_at_hours: u32::MAX - 5,
            })
        );
        assert_eq!(schedule.next(), None);
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn labels_include_the_upper_boundary() {
        let schedule = MaintenanceSchedule::new(100, 50, 5).unwrap();
        assert_eq!(
            due_labels_within(schedule, 200),
            ["#1 at 100 FH", "#2 at 150 FH", "#3 at 200 FH"]
        );
    }

    #[test]
    fn labels_can_be_empty_before_the_first_event() {
        let schedule = MaintenanceSchedule::new(100, 50, 3).unwrap();
        assert!(due_labels_within(schedule, 99).is_empty());
    }
}
