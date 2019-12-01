pub fn first(contents: &String) -> u32 {
    contents
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .map(|m| (m / 3) - 2u32)
        .sum::<u32>()
}

pub fn second(contents: &String) -> i32 {
    contents
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .map(|m| rocket_equation(m))
        .sum::<i32>()
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
