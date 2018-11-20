//use std::convert::*;
//use std::sync::mpsc;
//use std::thread;

//use rayon::prelude::*;
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


/// Labeled ScheduleabelOption, used in solver.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct InfoScheduleableOption<'option_lifetime> {
    inner: &'option_lifetime ScheduleableOption,
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

    /// Labels all of the ScheduleableOptions in self.options
    pub fn label_options(&self) -> Vec<InfoScheduleableOption> {
        self.options
            .iter()
            .map(|so| {InfoScheduleableOption{
                inner: so,
                start: self.start,
                end: self.get_end(),
            }})
            .collect()
    }

    /// Returns true if none of the ScheduleableOptions conflict with themselves.
    pub fn is_valid(&self) -> bool {
        self.label_options().iter().all(|iso| iso.is_valid())
    }

}

impl<'a> InfoScheduleableOption<'a> {

    fn conflict(&self, other: &Self) -> bool {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if end < start {false}
        else {
            let duration = end - start;
            !self.inner.events.iter()
                .any(|event| {
                    other.inner.events.iter()
                        .any(|event2| {
                            event.contains_between(0, event2.offset, duration) ||
                            event.contains_between(0, event2.get_end(), duration)
                        })
                })
        }
    }

    /// A ScheduleableOption is valid if none of its events conflict with each other.
    /// This is unfortunately an O(n^2) operation.
    fn is_valid(&self) -> bool {
        self.inner.events.iter()
            .map(|event| (event, &self.inner.events))
            .all(|(event, ref vec)| {
                !vec.iter()
                    .any(|event2| {
                        event.contains_between(0, event2.offset, self.end-self.start) ||
                        event.contains_between(0, event2.get_end(), self.end-self.start)
                    })
            })
    }
}


impl Event {
    /// time is in unix time.
    pub fn contains(&self, time: u64, during: &Scheduleable) -> bool {
        return self.contains_between(during.start, time, during.get_end());
    }

    pub fn get_end(&self) -> u64 {self.offset + self.duration}

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