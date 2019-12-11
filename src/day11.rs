use crate::infra::Problem;

pub struct Day11;

impl Problem<String, String, (), ()> for Day11 {
    fn day() -> u8 {
        11
    }
    fn first(contents: String) -> () {
        println!("{}", contents);
    }
    fn second(contents: String) -> () {
        println!("{}", contents);
    }
}
