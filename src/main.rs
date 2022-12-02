#![feature(option_result_contains)]

mod day_one;
mod day_two;

use day_one::{day_one_part_one, day_one_part_two};
use day_two::day_two_part_one;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Day 1 Part 1: {}", day_one_part_one(&args[1])?);
    println!("Day 1 Part 2: {}", day_one_part_two(&args[1])?);
    println!("Day 2 Part 1: {}", day_two_part_one(&args[2])?);
    Ok(())
}
