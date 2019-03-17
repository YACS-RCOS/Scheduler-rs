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
            // TODO Maybe this should just be a subroutine?
            // Seems unnecessary to make it recursive, and we just incur more
            // overhead from the match statement
            /*
            Filter out any schedule option that conflict with anything in the current
            list of schedule_options.
            Push all the schedule options that don't conflict with the current schedule
            onto clones of the current schedule and return all of the modified clones.
            */
            schedules // TODO Maybe this could be simpler? i.e. do we need to label options every time?
                .last()
                .unwrap()
                .label_options()
                .iter()
                .filter(|option| {
                    !current
                        .iter()
                        .any(|current_option| current_option.conflict(option))
                })
                .map(|option| {
                    let mut vec = current.clone();
                    vec.push(option.clone());
                    vec
                })
                .collect()
        }
        n => {
            // TODO Is this correct?
            let mut possible_schedules = vec![];
            for i in 0..n {
                // TODO Is this correct?
                let mut next_layer = vec![];
                for schedule in possible_schedules {
                    next_layer.append(&mut solve_rec(&schedules[i..i + 1], schedule));
                }
                possible_schedules = next_layer;
            }
            possible_schedules
        }
    }
}
