//use rayon::prelude::*;
//use rayon::iter::ParallelIterator;
use std::thread;
use std::sync::mpsc;

use std::convert::*;
use std::cmp::{min, max};

#[derive(Eq, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Event {
    pub uuid: String,
    /// Offset from the start of the owning scheduleable.
    pub offset: u64,
    pub duration: u64,
    pub repeat: u64,
}

#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct ScheduleableOption {
    pub uuid: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct Scheduleable {
    pub uuid: String,
    pub start: u64,
    pub duration: u64,
    pub options: Vec<ScheduleableOption>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct InfoScheduleableOption {
    inner: ScheduleableOption,
    start: u64,
    end: u64,
}

impl ::std::cmp::PartialEq for Scheduleable {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl ::std::cmp::PartialEq for ScheduleableOption {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl ::std::cmp::PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl Scheduleable {
    pub fn get_end(&self) -> u64 {self.start + self.duration}

    fn label_options(&self) -> Vec<InfoScheduleableOption> {
        self.options
            .iter()
            .map(|so| {InfoScheduleableOption{
                inner: *so,
                start: self.start,
                end: self.get_end(),
            }})
            .collect()
    }
}

impl InfoScheduleableOption {

    fn conflict(&self, other: &Self) -> bool {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if end < start {false}
        else {
            let mut current = start;
            true
        }
    }

    /// A ScheduleableOption is valid if none of its events conflict with each other.
    fn is_valid(&self) -> bool {
        self.inner.events.iter()
            .map(|event| (event, &self.inner.events))
            .all(|(event, ref vec)| {
                !vec.iter()
                    .any(|event2| {
                        event.contains_between()
                    })
            })
    }
}

impl ScheduleableOption {

}

impl Event {
    /// time is in unix time.
    pub fn contains(&self, time: u64, during: &Scheduleable) -> bool {
        return self.contains_between(during.start, time, during.get_end());
    }

    fn contains_between(&self, start: u64, time: u64, end: u64) -> bool {
        let mut mut_start = self.offset + start;
        while mut_start < end {
            if mut_start <= time && mut_start + self.duration > time { return true }
                else if self.repeat != 0 { mut_start += self.repeat; }
                    else { return false }
        }
        false
    }

}