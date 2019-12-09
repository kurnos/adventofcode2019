use crate::computer::{parse_memory, Computer};

pub fn first(contents: &String) -> i128 {
    Computer::run(parse_memory(contents), vec![1]).output[0]
}

pub fn second(contents: &String) -> i128 {
    Computer::run(parse_memory(contents), vec![2]).output[0]
}