use crate::computer::Computer;
use crate::infra::Problem;

pub struct Day5;

impl Problem<String, String, i32, i32, 5> for Day5 {
    fn first(contents: String) -> i32 {
        let mut c = Computer::from_str(&contents);
        c.run();
        c.run_with_input(1);
        c.last().unwrap()
    }
    fn second(contents: String) -> i32 {
        let mut c = Computer::from_str(&contents);
        c.run();
        c.run_with_input(5);
        (&mut c).next().unwrap()
    }
}
