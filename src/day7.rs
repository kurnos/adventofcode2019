use crate::computer::{parse_memory, Computer, StepResult};
use crate::utils::permutations;
use rayon::prelude::*;
use std::collections::VecDeque;

pub fn first(contents: &String) -> i32 {
    let mem = parse_memory(contents);
    permutations(vec![0, 1, 2, 3, 4])
        .into_par_iter()
        .map(|phases| trial(&mem, phases))
        .max()
        .unwrap()
}

fn trial(mem: &Vec<i32>, phases: Vec<i32>) -> i32 {
    let a = Computer::run_from(mem.clone(), vec![phases[0], 0]).output[0];
    let b = Computer::run_from(mem.clone(), vec![phases[1], a]).output[0];
    let c = Computer::run_from(mem.clone(), vec![phases[2], b]).output[0];
    let d = Computer::run_from(mem.clone(), vec![phases[3], c]).output[0];
    let e = Computer::run_from(mem.clone(), vec![phases[4], d]).output[0];
    e
}

pub fn second(contents: &String) -> i32 {
    let mem = parse_memory(contents);
    permutations(vec![5, 6, 7, 8, 9])
        .into_par_iter()
        .map(|phases| feedback_trial(&mem, phases))
        .max()
        .unwrap()
}

fn feedback_trial(mem: &Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut thrusters = VecDeque::from(vec![
        Computer::from_memory(mem.clone()),
        Computer::from_memory(mem.clone()),
        Computer::from_memory(mem.clone()),
        Computer::from_memory(mem.clone()),
        Computer::from_memory(mem.clone()),
    ]);

    for i in 0..5 {
        thrusters[i].input.push_back(phases[i]);
    }

    thrusters[0].input.push_back(0);
    loop {
        let mut state = thrusters.pop_front().unwrap();
        while state.output.len() == 0 {
            if let StepResult::Terminated = state.step() {
                return state.input.pop_front().unwrap();
            }
        }
        thrusters[0]
            .input
            .push_back(state.output.pop_front().unwrap());
        thrusters.push_back(state);
    }
}
