use crate::model::Scheduleable;
use crate::model::InfoScheduleableOption;
use crate::model::ScheduleableOption;
use crate::model::TimeUnit;
use crate::model::Event as RepeatEvent;

use std::thread;
use std::sync::mpsc;

use std::sync::Arc;

use crossbeam::channel;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq)]
struct Event {
    start: TimeUnit,
    duration: TimeUnit,
}

impl Event {
    fn new(start: TimeUnit, dur: TimeUnit) -> Self {
        Event {start, duration: dur}
    }
    fn end(&self) -> TimeUnit {self.start+self.duration}

    fn conflict(a: &Self, b: &Self) -> bool {
        (a.start >= b.start && a.start <= b.end()) ||
        (b.start >= a.start && b.start <= a.end())
    }

    fn is_before(&self, b: &Self) -> bool {
        self.end() < b.start
    }

    fn is_after(&self, b: &Self) -> bool {
        b.is_before(self)
    }
}

type Element = Vec<ElementOption>;
type ElementOption = Vec<Event>;

// O(n log n)
fn eo_sort_rev(mut eo: ElementOption) -> ElementOption {
    eo.sort_unstable_by_key(|ev| ev.start);
    eo.reverse();
    eo
}

fn eo_conflict_base(eo1: &ElementOption, eo2: &ElementOption) -> bool {
    let mut sorted1 = eo_sort_rev(eo1.clone());
    let mut sorted2 = eo_sort_rev(eo1.clone());
    return eo_conflict(&mut sorted1, &mut sorted2);
}

// for lists of lengths n and m, this function is O(max(n,m)) (?)
// this function assumes that both arguments are in reverse sorted order by
// event start time.
fn eo_conflict(eo1: &mut ElementOption, eo2: &mut ElementOption) -> bool {
    let oa = eo1.pop();
    let ob = eo2.pop();
    if oa.is_none() || ob.is_none() {return false;}
    else {
        let a = oa.unwrap();
        let b = ob.unwrap();
        if Event::conflict(&a,&b) {return true;}
        else if a.is_before(&b) {
            eo2.push(b);
            return eo_conflict(eo1, eo2);
        }
        else if a.is_after(&b) {
            eo1.push(a);
            return eo_conflict(eo1, eo2);
        } else {panic!("could not determine ElementOption conflict.");}
    }
}



/// Returns a vector of all the possible non-conflicting (partial and full)
/// sets of schedules.
pub fn solve(scheduleables: Vec<Scheduleable>) -> Vec<Vec<ScheduleableOption>> {

    unimplemented!()
}
