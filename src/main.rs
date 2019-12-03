use std::fs;
use std::env;
use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    let t0 = Instant::now();

    if day.unwrap_or(1) == 1 {
        let contents = fs::read_to_string("resources/day1.txt")?;
        println!("Day1a: {}", day1::first(&contents));
        println!("Day1b: {}", day1::second(&contents));
    }

    if day.unwrap_or(2) == 2 {
        let contents = fs::read_to_string("resources/day2.txt")?;
        println!("Day2a: {:?}", day2::first(&contents));
        println!("Day2b: {}", day2::second(&contents));
    }

    if day.unwrap_or(3) == 3 {
        let contents = fs::read_to_string("resources/day3.txt")?;
        println!("Day3a: {:?}", day3::first(&contents));
        println!("Day3b: {}", day3::second(&contents));
    }

    let elapsed = t0.elapsed();
    println!("Time taken: {:?}", elapsed);

    Ok(())
}
