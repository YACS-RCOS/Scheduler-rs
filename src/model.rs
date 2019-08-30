use std::cmp::{min, max};

/// Time type that we use, currently corresponds to seconds, or unix time.
pub type TimeUnit = u64;

/// An event.
///
/// Ex:
/// - A club meeting is an event.
/// - A class in a course is an event.
/// - A swimming meet is an event.
/// - Fencing practice is an event.
#[derive(Eq, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Event {
    pub uuid: String,
    /// Offset from the start of the owning scheduleable.
    pub offset: TimeUnit,
    pub duration: TimeUnit,
    /// Time from start -> start of next event
    pub repeat: TimeUnit,
}

/// An option for a scheduleable.
///
/// Ex. A course is a scheduleable. A section of that course is one of its
/// scheduleable options.
#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct ScheduleableOption {
    pub uuid: String,
    pub events: Vec<Event>,
}

/// Something which can be scheduled.
///
/// Ex:
/// - A course is a scheduleable.
/// - A a club is a scheduleable.
/// - A sport is a scheduleable.
#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct Scheduleable {
    pub uuid: String,
    pub start: TimeUnit,
    /// Duration of this scheduleable.
    /// This should be at least the difference between the start of this
    /// scheduleable and the end of the last event or event repeat.
    pub duration: TimeUnit,
    pub options: Vec<ScheduleableOption>,
}


/// Labeled ScheduleabelOption, used in solver.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct InfoScheduleableOption<'option_lifetime> {
    pub inner: &'option_lifetime ScheduleableOption,
    pub(crate) start: TimeUnit,
    pub(crate) end: TimeUnit,
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
    /// This scheduleables start + duration.
    pub fn get_end(&self) -> TimeUnit {self.start + self.duration}

    /// Labels all of the ScheduleableOptions in self.options
    pub fn label_options(&self) -> Vec<InfoScheduleableOption> {
        self.options
            .iter()
            .map(|scheduleable_option| {InfoScheduleableOption{
                inner: scheduleable_option,
                start: self.start,
                end: self.get_end(),
            }})
            .collect()
    }

    /// Returns true if none of the ScheduleableOptions conflict with themselves.
    pub fn is_valid(&self) -> bool { self.label_options().iter().all(|iso| iso.is_valid()) }

}

impl<'a> InfoScheduleableOption<'a> {

    /// Check if two InfoSceduleableOptions conflict.
    pub fn conflict(&self, other: &Self) -> bool {
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
    /// Check if this event or any of its repetitions within `during`
    /// contain `time`.
    pub fn contains(&self, time: TimeUnit, during: &Scheduleable) -> bool {
        self.contains_between(during.start, time, during.get_end())
    }

    /// This event's offset + duration.
    pub fn get_end(&self) -> TimeUnit {self.offset + self.duration}


    fn contains_between(&self, start: TimeUnit, time: TimeUnit, end: TimeUnit) -> bool {
        let mut mut_start = self.offset + start;
        while mut_start < end {
            if mut_start <= time && mut_start + self.duration > time { return true }
                else if self.repeat != 0 { mut_start += self.repeat; }
                    else { return false }
        }
        false
    }

}