use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;

pub struct Day5;

impl Problem<String, String, i32, i32> for Day5 {
    fn day() -> u8 {
        5
    }
    fn first(contents: String) -> i32 {
        Computer::run_from(parse_memory(&contents), vec![1])
            .output
            .into_iter()
            .last()
            .unwrap()
    }
    fn second(contents: String) -> i32 {
        Computer::run_from(parse_memory(&contents), vec![5]).output[0]
    }
}
