use super::{ TimeUnit };
use std::cmp::PartialEq;

/// An event in a schedule. Conceptually, this is an ordered
/// pair of integers, the first corresponding to the beginning of
/// event, the second the end time. To save space and reduce
/// runtime, we store events as a start time, duration, and
/// a repeat interval. Thus, an event struct can represent
/// multiple 'Events'.
///
/// Note that the internal representation is inclusive on the
/// start and exclusive on the end. That is, the end of the event
/// is the start + duration, and the end of one event can equal
/// the `start` of another without being incompatible
///
#[derive(Clone, Copy, Debug, Eq)]
pub struct Event {
    /// Start of event
    start: TimeUnit,
    /// Duration of the event
    duration: TimeUnit,
}

impl PartialEq for Event {

    fn eq(&self, other: &Self) -> bool {
        other.start == self.start
            && other.duration == self.duration
    }
}

impl Event {

    /// Create a new event
    pub fn new(start: TimeUnit, duration: TimeUnit) -> Event {
        Event { start, duration }
    }

    /// Returns true if this event contains data which is valid
    pub fn is_valid(&self) -> bool {
        true
    }

    /// Returns the time that this event starts at
    pub fn start(&self) -> TimeUnit {
        self.start
    }

    /// Returns the time this event goes on for
    pub fn duration(&self) -> TimeUnit {
        self.duration
    }

    /// Returns the time that this event ends at
    pub fn end(&self) -> TimeUnit {
        self.duration + self.start
    }
}

