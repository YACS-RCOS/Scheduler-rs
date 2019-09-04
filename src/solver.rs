use crate::model::{
    TimeUnit,
    ApiScheduleable,
    ApiScheduleableOption
};

use std::{
    collections::{
        HashMap,
        HashSet
    },
    ops::Range
};

/// Serial Identifiers/Indexes for Elements and ElementOptions.
type ID = u32;

#[derive(Debug, Clone, Hash)]
struct Event {
    inner: Range<TimeUnit>,
    /// ID of the ElementOption of this event
    id: ID,
}

impl Event {
    /// Construct new event
    pub const fn new(start: TimeUnit, dur: TimeUnit, id: ID) -> Self {
        Event {
            inner: start..start+dur,
            id
        }
    }

    /// If either event contains the other ones start then they conflict.
    pub fn conflict(a: &Self, b: &Self) -> bool {
        a.inner.contains(&b.inner.start) ||
        b.inner.contains(&a.inner.start)
    }

    #[allow(missing_docs)]
    #[inline]
    pub const fn is_before(&self, other: &Self) -> bool {
        other.inner.start >= self.inner.end
    }

    #[allow(missing_docs)]
    #[inline]
    pub const fn is_after(&self, other: &Self) -> bool {
        self.inner.start >= other.inner.end
    }
}

#[derive(Debug)]
struct ElementOption {
    conflicts: HashSet<ID>,
    inner: Vec<Event>,
    id: ID,
    elem_id: ID,
}

#[derive(Debug)]
struct Element {
    id: ID,
    option_range: Range<ID>,
}

#[derive(Debug)]
pub struct Solver {
    next_id: ID,
    /// Map serial IDs to UUIDs.
    id_map: HashMap<ID, String>,
    elements: Vec<Element>,
    options: Vec<ElementOption>,

}

impl Solver {
    /// Construct new Solver.
    pub fn new() -> Self {
        Solver {
            next_id: 0,
            id_map: HashMap::new(),
            elements: Vec::new(),
            options: Vec::new(),
        }
    }

    /// Add scheduleable from Api into this Solver.
    pub fn add_scheduleable(&mut self, scheduleable: ApiScheduleable) {
        unimplemented!()
    }
}

/// Returns a vector of all the possible non-conflicting (partial and full)
/// sets of schedules.
pub fn solve(scheduleables: Vec<ApiScheduleable>) -> Vec<Vec<ApiScheduleableOption>> {
    unimplemented!()
}
