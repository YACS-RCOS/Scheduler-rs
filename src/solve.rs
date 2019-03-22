use crate::models::{Schedule};

/// Naively solves for all possibilities.
/// We can later filter the output to what we want,
/// but this naive implementation is guarranteed to be correct
pub fn solve_naive(schedules: &Vec<Schedule>) -> Vec<Schedule> {
    let mut output = schedules.clone();

    for input_schedule in schedules {
        let mut running = vec![];
        for schedule in output.iter() {
            let mut current = Schedule::compose(&vec![&schedule, &input_schedule]);
            if current.is_valid() {
                running.push(current);
            }
        }
        output.append(&mut running);
    }
    output
}


