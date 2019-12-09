use crate::computer::{parse_memory, run};

pub fn first(contents: &String) -> i128 {
    let memory = parse_memory(contents);
    let x = run(memory, vec![1]);
    x.output[0]
}

pub fn second(contents: &String) -> i128 {
    let memory = parse_memory(contents);
    let x = run(memory, vec![2]);
    x.output[0]
}