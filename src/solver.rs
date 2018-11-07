use super::model::*;
use std::collections::HashMap;
use super::rayon::prelude::*;

pub fn solve(scheduleables: Vec<Scheduleable>) -> Vec<ScheduleableOption> {
    unimplemented!()
}


pub fn get_conflicts(scheduleables: Vec<Scheduleable>) ->
        HashMap<ScheduleableOption, Vec<ScheduleableOption>> {
    let mut map: HashMap<ScheduleableOption, Vec<ScheduleableOption>> =
        HashMap::new();
    let mut curr_time = 0u64;

}