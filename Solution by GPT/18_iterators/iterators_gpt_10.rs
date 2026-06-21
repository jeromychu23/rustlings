// GPT Rustlings extension
// Topic: 18 Iterators
// Difficulty: Advanced
// Scenario: Recurring maintenance schedule

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
        if interval_hours == 0 {
            Err(ScheduleError::ZeroInterval)
        } else {
            Ok(Self {
                next_due: Some(first_due_hours),
                interval_hours,
                remaining: count,
                next_sequence: 1,
            })
        }
    }
}

impl Iterator for MaintenanceSchedule {
    type Item = DueEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            self.next_due = None;
            return None;
        }

        let due_at_hours = self.next_due?;
        let event = DueEvent {
            sequence: self.next_sequence,
            due_at_hours,
        };

        self.remaining -= 1;
        self.next_sequence += 1;

        // The current event is valid. Failure to calculate the next one makes
        // the iterator exhausted without overflowing.
        if self.remaining == 0 {
            self.next_due = None;
        } else {
            self.next_due = due_at_hours.checked_add(self.interval_hours);
            if self.next_due.is_none() {
                self.remaining = 0;
            }
        }

        Some(event)
    }
}

fn due_labels_within(schedule: MaintenanceSchedule, max_hours: u32) -> Vec<String> {
    schedule
        .take_while(|event| event.due_at_hours <= max_hours)
        .map(|event| format!("#{} at {} FH", event.sequence, event.due_at_hours))
        .collect()
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
