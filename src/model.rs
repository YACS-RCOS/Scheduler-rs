pub use super::*;

pub use std::time::*;


#[derive(Eq, Clone)]
pub struct Event {
    pub uuid: Uuid,
    pub start: Duration,
    pub duration: Duration,
    pub repeat: Option<Duration>,
}


#[derive(Clone,Eq)]
pub struct ScheduleAbleOption {
    pub uuid: Uuid,
    pub events: Vec<Event>,
}

#[derive(Clone, Eq)]
pub struct ScheduleAble {
    pub uuid: Uuid,
    pub start: SystemTime,
    pub duration: Duration,
    pub options: Vec<ScheduleAbleOption>,
}

impl ::std::cmp::PartialEq for ScheduleAble {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl ::std::cmp::PartialEq for ScheduleAbleOption {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl ::std::cmp::PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}