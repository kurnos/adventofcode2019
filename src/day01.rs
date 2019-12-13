use crate::infra::Problem;

pub struct Day1;

impl Problem<String, String, u32, i32> for Day1 {
    fn day() -> u8 {
        1
    }
    fn first(contents: String) -> u32 {
        contents
            .lines()
            .map(|s| s.parse::<u32>().unwrap())
            .map(|m| (m / 3) - 2u32)
            .sum::<u32>()
    }
    fn second(contents: String) -> i32 {
        contents
            .lines()
            .map(|s| s.parse::<u32>().unwrap())
            .map(rocket_equation)
            .sum::<i32>()
    }
}

fn rocket_equation(mass: u32) -> i32 {
    let mut fuel = 0i32;
    let mut mass: i32 = (mass as i32 / 3) - 2;
    while mass > 0 {
        fuel += mass;
        mass = (mass / 3) - 2;
    }
    fuel
}
