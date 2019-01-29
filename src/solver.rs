use super::model::*;

/// Returns a vector of all the possible non-conflicting sets of schedules.
pub fn solve(scheduleables: Vec<Scheduleable>) -> Vec<Vec<ScheduleableOption>> {
    solve_rec(scheduleables.as_slice(), vec![])
        .iter()
        .map(|v| v.iter()
            .map(|iso| iso.inner.clone())
            .collect())
        .collect()
}

/// Recursively solve for all possible schedules.
fn solve_rec<'lifetime>(scheduleables: &'lifetime [Scheduleable], current: Vec<InfoScheduleableOption<'lifetime>>)
    -> Vec<Vec<InfoScheduleableOption<'lifetime>>> {
    match scheduleables.len() {
        0 => vec![current],
        1 => {
            /*
            Filter out any scheduleable option that conflict with anything in the current
            list of scheduleable_options.
            Push all the scheduleable options that don't conflict with the current schedule
            onto clones of the current schedule and return all of the modified clones.
            */
            scheduleables.last()
                .unwrap()
                .label_options()
                .iter()
                .filter(|scheduleable_option| !current.iter()
                    .any(|currently_selected_scheduleable_option|
                        currently_selected_scheduleable_option.conflict(scheduleable_option)))
                .map(|non_conflicting_scheduleable_option| {
                    let mut vec = current.clone();
                    vec.push(non_conflicting_scheduleable_option.clone());
                    vec
                })
                .collect()
        },
        n => {
            /*

            */
            let mut possible_schedules = vec![];
            for i in 0..n {
                let mut next_layer_of_possible_schedules = vec![];
                for schedule in possible_schedules {
                    let mut some_possible_schedules = solve_rec(&scheduleables[i..i+1], schedule);
                    next_layer_of_possible_schedules.append(&mut some_possible_schedules);
                }
                possible_schedules = next_layer_of_possible_schedules;
            }
            possible_schedules
        },
    }
}