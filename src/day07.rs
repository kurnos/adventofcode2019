use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;
use crate::utils::permutations;
use rayon::prelude::*;

pub struct Day7;

impl Problem<String, String, i32, i32, 7> for Day7 {
    fn first(contents: String) -> i32 {
        let mem = crate::computer::parse_memory(&contents);
        permutations(vec![0, 1, 2, 3, 4])
            .into_par_iter()
            .map(|phases| trial(&mem, phases))
            .max()
            .unwrap()
    }
    fn second(contents: String) -> i32 {
        let mem = parse_memory(&contents);
        permutations(vec![5, 6, 7, 8, 9])
            .into_par_iter()
            .map(|phases| feedback_trial(&mem, phases))
            .max()
            .unwrap()
    }
}

fn trial(mem: &[i32], phases: Vec<i32>) -> i32 {
    let mut token = 0;
    for phase in phases {
        token = Computer::from_memory(mem.to_owned())
            .run_through(vec![phase, token])
            .into_iter()
            .last()
            .unwrap();
    }
    token
}

fn feedback_trial(mem: &[i32], phases: Vec<i32>) -> i32 {
    let mut thrusters = [
        Computer::from_memory(mem.to_owned()),
        Computer::from_memory(mem.to_owned()),
        Computer::from_memory(mem.to_owned()),
        Computer::from_memory(mem.to_owned()),
        Computer::from_memory(mem.to_owned()),
    ];

    for i in 0..5 {
        thrusters[i].run();
        thrusters[i].run_with_input(phases[i]);
    }

    let mut token = 0;
    for i in std::iter::Iterator::cycle(0..5) {
        if thrusters[i].is_terminated() {
            return token;
        }
        thrusters[i].run_with_input(token);
        token = (&mut thrusters[i]).next().unwrap()
    }
    0
}
