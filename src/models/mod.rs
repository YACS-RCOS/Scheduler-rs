/// Time type, whose value corresponds to a Unix Timestamp
pub type TimeUnit = u64;
/// Time unit that represents forever from now
pub const FOREVER: TimeUnit = std::u64::MAX;
/// A second
pub const SECOND: TimeUnit = 1;
/// A minute
pub const MINUTE: TimeUnit = 60 * SECOND;
/// An hour
pub const HOUR: TimeUnit = 60 * MINUTE;
/// A week
pub const WEEK: TimeUnit = 7 * HOUR;

mod event;
mod schedule;
pub use crate::model::*;
pub use crate::model::Event as ApiEvent;
pub use self::event::Event;
pub use self::schedule::Schedule;
