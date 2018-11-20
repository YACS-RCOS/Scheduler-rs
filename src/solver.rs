use super::model::*;

/// Returns a vector of all the possible non-conflicting sets of schedules.
pub fn solve(scheduleables: Vec<Scheduleable>) -> Vec<Vec<ScheduleableOption>> {
    if scheduleables.iter().all(|s| s.is_valid()) {
        let mut result: Vec<Vec<ScheduleableOption>> = vec![];

    }
    return vec![]
}

fn get_possible(current: &Vec<ScheduleableOption>)