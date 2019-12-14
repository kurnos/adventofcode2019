use crate::computer::Computer;
use crate::infra::Problem;

pub struct Day5;

impl Problem<String, String, i32, i32> for Day5 {
    fn day() -> u8 {
        5
    }
    fn first(contents: String) -> i32 {
        let mut c = Computer::from_str(&contents);
        c.run();
        c.run_with_input(1);
        c.iter().last().unwrap()
    }
    fn second(contents: String) -> i32 {
        let mut c = Computer::from_str(&contents);
        c.run();
        c.run_with_input(5);
        c.iter().next().unwrap()
    }
}
