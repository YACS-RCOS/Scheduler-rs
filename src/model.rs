use rayon::prelude::*;
use std::convert::*;

#[derive(Eq, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Event {
    pub uuid: String,
    pub start: u64,
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
    pub fn get_options_at_time(&self, time: u64) -> Vec<ScheduleableOption> {

    }

    pub fn get_next_time(&self, from: u64) -> Option<u64> {
        if from < start { self.get_next_time(self.start) }
        else if from > self.start + self.duration { None }
        else {

        }
    }

    pub fn get_end(&self) -> u64 {self.start + self.duration}
}

impl Event {
    /// time is from Scheduleable.start.
    /// max is Scheduleable.get_end()
    pub fn contains(&self, time: u64, max: u64) -> bool {
        let mut mut_start = self.start;
        while mut_start < max {
            if mut_start <= time && mut_start + self.duration > time { true }
            else if self.repeat != 0 { mut_start += self.repeat; }
            else { false }
        }
        false
    }

    pub fn conflict(a: &Self, b: &Self, ) -> bool {

    }
}