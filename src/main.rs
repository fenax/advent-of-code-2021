extern crate regex;
extern crate itertools;
mod parser;
mod formater;
mod day01;
mod day02;

fn main() -> Result<(), std::io::Error> {
    day01::run()?;
    day02::run()?;
    Ok(())  
}
