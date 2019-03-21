use super::{/*TimeUnit,*/ Event};
use std::cmp::PartialEq;
use itertools::Itertools;
use std::collections::HashMap;

/// State of a schedule, i.e. whether the schedule
/// has been validated, sorted, etc.
/// Hierarchy is:
/// Unchecked means haven't been sorted or validated
/// Sorted means haven't validated, but have sorted
/// Invalid/Valid means has been sorted, and validated
#[derive(Clone, Copy, Debug, Eq)]
enum ScheduleState {
    Invalid,
    Valid,
    Unchecked,
    Sorted,
}

impl PartialEq for ScheduleState {

    fn eq(&self, other: &Self) -> bool {
        return self == other
    }
}


/// An unordered set of events.
/// Underlying data structure is sorted
/// for runtime efficiency
///
#[derive(Clone, Debug, Eq)]
pub struct Schedule {
    /// Universally unique id for this schedule
    pub uuids: Vec<String>,
    /// Events in this schedule
    pub events: Vec<Event>,
    state: ScheduleState,
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        self.uuids == other.uuids
    }
}

impl Schedule {

    /// New Schedule Object
    pub fn new(uuid: String, events: Vec<Event>) -> Schedule {
        let uuids = vec![uuid];
        Schedule {
            uuids,
            events,
            state: ScheduleState::Unchecked,
        }
    }

    /// New Schedule as composition of other schedules
    /// Schedule returned will already be sorted by time,
    /// but will NOT be validated.
    pub fn compose(schedules: &Vec<Schedule>) -> Schedule {

        let mut uuid_count = 0;
        let mut unique_uuids: HashMap<String,u16> = HashMap::new();
        let mut valid = true;

        let uuids: Vec<String> = schedules.iter()
            .map(|sched| {
                if sched.state == ScheduleState::Invalid {
                    valid = false;
                }
                let id_list = sched.uuids.clone();
                for id in id_list.iter() { // Store unique id's found
                    unique_uuids.insert(id.to_string(), 0);
                }
                uuid_count += id_list.len(); // to compare against total found
                id_list
            }).kmerge()
            .collect();

        if uuid_count > uuids.len() {
            valid = false;
        }

        if valid {
            let events: Vec<Event> = schedules.iter()
                .map(|sched| sched.events.clone())
                .kmerge_by(|a, b| a.start() < b.start())
                .collect();

            Self {
                uuids,
                events,
                state: ScheduleState::Sorted,
            }
        } else {
            Self {
                uuids,
                events: Vec::new(),
                state: ScheduleState::Invalid,
            }
        }
    }

    /// Returns whether or not this object is valid
    pub fn is_valid(&mut self) -> bool {
        use ScheduleState::*;
        match self.state {
            Invalid => false,
            Valid => true,
            Unchecked => {
                self.sort_items();
                self.validate_sorted()
            },
            Sorted => {
                self.validate_sorted()
            }
        }
    }

    /// Validates a sorted schedule
    fn validate_sorted(&mut self) -> bool {
        use ScheduleState::*;
        let mut valid = true;
        let events = &self.events;

        // Pairwise iterate through and check if the event finish times are in
        // chronological order
        for (f1, b2) in events.windows(2).map(|a| (a[0].end(), a[1].start())) {
            if f1 > b2 {
                valid = false;
                break;
            }
        }

        self.state = if valid { Valid } else { Invalid };
        valid
    }

    fn sort_items(&mut self) {
        // TODO
        self.events.sort_by(|a, b| a.start().cmp(&b.start()));
    }

    /// Returns whether or not this object is compatible
    /// with another object of the same type
    pub fn compatible_with(&self, _other: &Self) -> bool {
        // TODO
        false
    }

}
