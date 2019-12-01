use std::fs;
use std::env;
mod day1;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    if day.unwrap_or(1) == 1 {
        let contents = fs::read_to_string("resources/day1.txt")?;
        println!("Day1a: {}", day1::first(&contents));
        println!("Day1b: {}", day1::second(&contents));
    }

    Ok(())
}
