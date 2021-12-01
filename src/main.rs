extern crate regex;
extern crate itertools;
mod parser;
mod day01;

use std::fs::read_to_string;
use std::time::Instant;
use std::vec::Vec;
use colored::*;

fn timed_run<F,T>(mut f: F)->T where F: FnMut()->T{
    let start = Instant::now();
    let x = f();
    let duration = start.elapsed();
    println!("{}",format!("  Timing : {:?}", duration).dimmed());
    x
}

macro_rules! one_day {
    ($day:ident) => 
   {
        println!("{}",stringify!($day).bold());

        let input = read_to_string(concat!(stringify!($day),".input"))?;
        $day::run(&input);

    };
}

pub fn print_header(num:usize){
    println!("Day {:2}", num);
}

pub fn print_parse(){
    print!("   {}","parsing".green().dimmed());
}

pub fn print_part_1(){
    print!("    {}","part 1".green());
}

pub fn print_part_2(){
    print!("    {}","part 2".green());
}

pub fn print_result<T:std::fmt::Display>(result:&T){
    println!("     {}",result);
}

fn main() -> Result<(), std::io::Error> {
//  one_day_with_input!(day15,"3,1,2");
    one_day!(day01);

    Ok(())  
}
