extern crate regex;
extern crate itertools;
mod parser;
mod day01;
mod day02;

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

pub fn print_single_parse<F,G,H,T,U,V>(num:usize, mut parser:F, mut part1:G, mut part2:H) -> Result<(), std::io::Error> 
where 
F: FnMut(&str)->T, 
G: FnMut(&T)->U, 
H: FnMut(&T)->V, 
U: std::fmt::Display, 
V: std::fmt::Display
{
    print_header(num);
    let file_data = read_to_string(format!("day{:02}.input",num))?;
    print_parse();
    let data = timed_run(||{parser(&file_data)});
    print_part_1();
    let result1 = timed_run(||{part1(&data)});
    print_result(&result1);
    print_part_2();
    let result2 = timed_run(||{part2(&data)});
    print_result(&result2);
    Ok(())     
}

fn print_header(num:usize){
    println!("{}",format!("Day {:2}", num).bold());
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
    day02::run()?;
    Ok(())  
}
