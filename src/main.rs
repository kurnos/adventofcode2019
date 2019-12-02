use std::fs;
use std::env;
mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    if day.unwrap_or(1) == 1 {
        let contents = fs::read_to_string("resources/day1.txt")?;
        println!("Day1a: {}", day1::first(&contents));
        println!("Day1b: {}", day1::second(&contents));
    }

    if day.unwrap_or(2) == 2 {
        let contents = fs::read_to_string("resources/day2.txt")?;
        //let contents = "1,0,0,0,99".to_string();
        //let contents = "2,3,0,3,99".to_string();
        //let contents = "2,4,4,5,99,0".to_string();
        //let contents = "1,1,1,4,99,5,6,0,99".to_string();
        println!("Day2a: {:?}", day2::first(&contents));
        println!("Day2b: {}", day2::second(&contents));
    }

    Ok(())
}
