extern crate regex;
extern crate itertools;
extern crate colored;
mod parser;
mod formater;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() -> Result<(), std::io::Error> {
    day01::run()?;
    day02::run()?;
    day03::run()?;
    day04::run()?;
    day05::run()?;
    day06::run()?;
    day07::run()?;
    day08::run()?;
    day09::run()?;
    day10::run()?;
    Ok(())  
}
