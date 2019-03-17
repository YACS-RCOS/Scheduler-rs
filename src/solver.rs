use crate::model::*;

/// Returns a vector of all the possible non-conflicting sets of schedules.
pub fn solve(schedules: Vec<ScheduleList>) -> Vec<Vec<ScheduleOption>> {
    solve_rec(schedules.as_slice(), vec![])
        .iter()
        .map(|v| v.iter().map(|iso| iso.inner.clone()).collect())
        .collect()
}

/// Recursively solve for all possible schedules.
fn solve_rec<'lifetime>(
    schedules: &'lifetime [ScheduleList],
    current: Vec<InfoScheduleOption<'lifetime>>,
) -> Vec<Vec<InfoScheduleOption<'lifetime>>> {
    match schedules.len() {
        0 => vec![current],
        1 => {
            /*
            Filter out any schedule option that conflict with anything in the current
            list of schedule_options.
            Push all the schedule options that don't conflict with the current schedule
            onto clones of the current schedule and return all of the modified clones.
            */
            schedules
                .last()
                .unwrap()
                .label_options()
                .iter()
                .filter(|schedule_option| {
                    !current.iter().any(|currently_selected_schedule_option| {
                        currently_selected_schedule_option.conflict(schedule_option)
                    })
                })
                .map(|non_conflicting_schedule_option| {
                    let mut vec = current.clone();
                    vec.push(non_conflicting_schedule_option.clone());
                    vec
                })
                .collect()
        }
        n => {
            let mut possible_schedules = vec![];
            for i in 0..n {
                let mut next_layer_of_possible_schedules = vec![];
                for schedule in possible_schedules {
                    let mut some_possible_schedules = solve_rec(&schedules[i..i + 1], schedule);
                    next_layer_of_possible_schedules.append(&mut some_possible_schedules);
                }
                possible_schedules = next_layer_of_possible_schedules;
            }
            possible_schedules
        }
    }
}
