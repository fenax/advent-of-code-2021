extern crate regex;
extern crate itertools;
mod parser;
mod formater;
mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<(), std::io::Error> {
    day01::run()?;
    day02::run()?;
    day03::run()?;
    day04::run()?;
    Ok(())  
}
