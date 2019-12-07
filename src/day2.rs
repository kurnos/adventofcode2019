use crate::computer::{parse_memory, run};
use rayon::prelude::*;

pub fn first(content: &String) -> i32 {
    let mut memory = parse_memory(content);
    memory[1] = 12;
    memory[2] = 2;
    run(memory, vec![]).memory[0]
}

pub fn second(content: &String) -> i32 {
    let initial = parse_memory(content);
    (0i32..10000)
        .into_par_iter()
        .find_first(|i| {
            let mut memory = initial.clone();
            memory[1] = i / 100;
            memory[2] = i % 100;
            run(memory, vec![]).memory[0] == 19690720
        })
        .unwrap()
}
