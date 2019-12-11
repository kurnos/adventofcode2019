use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;

pub struct Day9;

impl Problem<String, String, i64, i64> for Day9 {
    fn day() -> u8 {
        9
    }
    fn first(contents: String) -> i64 {
        Computer::run_from(parse_memory(&contents), vec![1]).output[0]
    }
    fn second(contents: String) -> i64 {
        Computer::run_from(parse_memory(&contents), vec![2]).output[0]
    }
}
