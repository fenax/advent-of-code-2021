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

pub fn print_single_parse<F,G,H,T,U,V>(num:usize, parser:F, mut part1:G, mut part2:H) 
where 
F: FnMut()->T, 
G: FnMut(&T)->U, 
H: FnMut(&T)->V, 
U: std::fmt::Display, 
V: std::fmt::Display
{
    print_header(num);
    print_parse();
    let data = timed_run(parser);
    print_part_1();
    let result1 = timed_run(||{part1(&data)});
    print_result(&result1);
    print_part_2();
    let result2 = timed_run(||{part2(&data)});
    print_result(&result2)
}

fn print_header(num:usize){
    println!("Day {:2}", num);
}

fn print_parse(){
    print!("   {}","parsing".green().dimmed());
}

fn print_part_1(){
    print!("    {}","part 1".green());
}

fn print_part_2(){
    print!("    {}","part 2".green());
}

fn print_result<T:std::fmt::Display>(result:&T){
    println!("     {}",result);
}

fn main() -> Result<(), std::io::Error> {
//  one_day_with_input!(day15,"3,1,2");
    one_day!(day01);

    Ok(())  
}
