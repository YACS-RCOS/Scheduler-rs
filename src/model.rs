use std::cmp::{min, max, PartialEq};

/// Time type that we use, currently corresponds to seconds, or unix time.
pub type TimeUnit = u64;

/// Something which can be scheduled.
///
/// Ex:
/// - A course.
/// - A club.
/// - A sport.
#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct ApiScheduleable {
    pub uuid: String,
    pub start: TimeUnit,
    /// Duration of this scheduleable.
    /// This should be at least the difference between the start of this
    /// scheduleable and the end of the last event or event repeat.
    pub duration: TimeUnit,
    pub options: Vec<ApiScheduleableOption>,
}

/// An option for an `ApiScheduleable`.
///
/// Ex.
/// A course is an `ApiScheduleable`.
/// A section of that course is one of its options.
#[derive(Clone, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct ApiScheduleableOption {
    pub uuid: String,
    pub events: Vec<ApiEvent>,
}

/// An event.
///
/// Ex:
/// - A club meeting.
/// - A class.
/// - A sports practice.
#[derive(Eq, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct ApiEvent {
    pub uuid: String,
    /// Offset from the start of the owning `ApiScheduleable`.
    pub offset: TimeUnit,
    pub duration: TimeUnit,
    /// Time from start of this event -> start of next event.
    pub repeat: TimeUnit,
}

// Operator overloads for `==` operator.

impl PartialEq for ApiScheduleable {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl PartialEq for ApiScheduleableOption {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl PartialEq for ApiEvent {
    fn eq(&self, other: &Self) -> bool {other.uuid == self.uuid}
}

impl ApiScheduleable {
    /// Get the end of an `ApiScheduleable`.
    #[inline]
    pub fn get_end(&self) -> TimeUnit {self.start + self.duration}
}

// other method implementations

impl ApiEvent {
    /// Check if this event or any of its repetitions within `during`
    /// contain `time`.
    pub fn contains(&self, time: TimeUnit, during: &ApiScheduleable) -> bool {
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