use crate::computer::Computer;
use crate::infra::Problem;

pub struct Day9;

impl Problem<String, String, i64, i64, 9> for Day9 {
    fn first(contents: String) -> i64 {
        Computer::from_str(&contents).run_through(vec![1])[0]
    }
    fn second(contents: String) -> i64 {
        Computer::from_str(&contents).run_through(vec![2])[0]
    }
}
