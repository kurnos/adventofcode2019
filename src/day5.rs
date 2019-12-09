use crate::computer::{parse_memory, Computer};

pub fn first(contents: &String) -> i128 {
    Computer::run(parse_memory(contents), vec![1])
        .output
        .into_iter()
        .last()
        .unwrap()
}

pub fn second(contents: &String) -> i128 {
    Computer::run(parse_memory(contents), vec![5]).output[0]
}
