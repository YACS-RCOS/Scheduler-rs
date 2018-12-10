use super::model::*;

/// Returns a vector of all the possible non-conflicting sets of schedules.
pub fn solve(scheduleables: Vec<Scheduleable>) -> Vec<Vec<ScheduleableOption>> {
    return solve_rec(scheduleables.as_slice(), vec![])
        .iter()
        .map(|v| v.iter()
            .map(|iso| iso.inner.clone())
            .collect())
        .collect()
}

/// Recursively solve for all possible schedules.
fn solve_rec<'lifetime>(scheds: &'lifetime [Scheduleable], current: Vec<InfoScheduleableOption<'lifetime>>)
    -> Vec<Vec<InfoScheduleableOption<'lifetime>>> {
    match scheds.len() {
        0 => vec![current],
        1 => {
            scheds.last()
                .unwrap()
                .label_options()
                .iter()
                .filter(|siso| !current.iter()
                    .any(|ciso| ciso.conflict(siso)))
                .map(|siso| {
                    let mut vec = current.clone();
                    vec.push(siso.clone());
                    vec
                })
                .collect()
        },
        n => {
            let mut res = solve_rec(&scheds[0..1], vec![]);
            for i in 1..n {
                let mut new_vec = vec![];
                for schedule in res {
                    new_vec.append(&mut solve_rec(&scheds[i..i+1], schedule));
                }
                res = new_vec;
            }
            res
        },
    }
}