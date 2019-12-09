use crate::computer::{parse_memory, run, run_instruction};
use crate::utils::permutations;
use rayon::prelude::*;
use std::collections::{VecDeque, HashMap};

pub fn first(contents: &String) -> i128 {
    let mem = parse_memory(contents);
    permutations(vec![0, 1, 2, 3, 4])
        .into_par_iter()
        .map(|phases| trial(&mem, phases))
        .max()
        .unwrap()
}

fn trial(mem: &Vec<i128>, phases: Vec<i128>) -> i128 {
    let a = run(mem.clone(), vec![phases[0], 0]).output[0];
    let b = run(mem.clone(), vec![phases[1], a]).output[0];
    let c = run(mem.clone(), vec![phases[2], b]).output[0];
    let d = run(mem.clone(), vec![phases[3], c]).output[0];
    let e = run(mem.clone(), vec![phases[4], d]).output[0];
    e
}

pub fn second(contents: &String) -> i128 {
    let mem = parse_memory(contents);
    permutations(vec![5, 6, 7, 8, 9])
        .into_par_iter()
        .map(|phases| feedback_trial(&mem, phases))
        .max()
        .unwrap()
}

fn feedback_trial(mem: &Vec<i128>, phases: Vec<i128>) -> i128 {
    let mut thrusters = VecDeque::from(vec![
        (0usize, mem.clone()),
        (0usize, mem.clone()),
        (0usize, mem.clone()),
        (0usize, mem.clone()),
        (0usize, mem.clone()),
    ]);

    for i in 0..5 {
        let (ic, mem) = &mut thrusters[i];
        *ic = run_instruction(
            *ic,
            mem,
            &mut HashMap::new(),
            &mut VecDeque::from(vec![phases[i]]),
            &mut VecDeque::new(),
            &mut 0
        )
        .unwrap();
    }

    let mut input = VecDeque::from(vec![0i128]);
    let mut output = VecDeque::new();
    loop {
        let (mut ic, mut mem) = thrusters.pop_front().unwrap();
        while output.len() == 0 {
            if let Some(next_ic) = run_instruction(ic, &mut mem, &mut HashMap::new(), &mut input, &mut output, &mut 0) {
                ic = next_ic;
            } else {
                return input.pop_front().unwrap();
            }
        }
        std::mem::swap(&mut input, &mut output);
        thrusters.push_back((ic, mem));
    }
}
