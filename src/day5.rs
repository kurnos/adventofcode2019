use crate::computer::{parse_memory, Computer};

pub fn first(contents: &String) -> i32 {
    Computer::run_from(parse_memory(contents), vec![1])
        .output
        .into_iter()
        .last()
        .unwrap()
}

pub fn second(contents: &String) -> i32 {
    Computer::run_from(parse_memory(contents), vec![5]).output[0]
}
