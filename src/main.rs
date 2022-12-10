extern crate colored;
extern crate im;
extern crate itertools;
extern crate regex;
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
mod day11;
mod day12;
mod day13;
mod day14;
mod formater;
mod parser;

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
    day11::run()?;
    day12::run()?;
    day13::run()?;
    day14::run()?;

    Ok(())
}
