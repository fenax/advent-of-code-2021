extern crate regex;
extern crate itertools;
mod parser;

mod puzzles;

use std::fs::read_to_string;
use std::time::Instant;
use puzzles::*;
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

        one_day_with_input!($day, read_to_string(concat!(stringify!($day),".input"))?);

    };
}

macro_rules! one_day_with_input {
    ($day:ident,$input:expr) => 
   {
        println!("{}",stringify!($day).bold());

        let i_string = $input;
        print!("     {}","parsing".green().dimmed());
        let input =timed_run(||{$day::Input::new(&i_string)});
       
        let mut part1 = $day::Part1::new(&input);
        let mut part2 = $day::Part2::new(&input);

        print!("      {}","part 1".green());
        timed_run(||{part1.resolve();});
        println!("    {}",part1);

        print!("      {}","part 2".green());
        timed_run(||{part2.resolve();});
        println!("    {}",part2);
        println!("");

    };
}
/*
fn group_tuples<A,B>(mut input: Vec<(A,B)>) -> Vec<(A,Vec<B>)>
where A:std::cmp::Ord +  Copy,
B:std::cmp::Ord + Copy,
{
    input.sort();
    let mut ret = Vec::new();
    let mut cur;
    let mut it = input.iter();
    match it.next(){
        None => return ret,
        Some(x) => {
            ret.push( (x.0,Vec::new()));
            let len = ret.len()-1;
            cur = &mut ret[len];
        }
    }
    for i in input.iter(){
        if i.0 == cur.0 {
            cur.1.push(i.1);
        }else{
            ret.push( (i.0,Vec::new()));
            let len = ret.len()-1;
            cur = &mut ret[len];
        }
    }
    ret
}
*/

fn main() -> Result<(), std::io::Error> {
//  one_day_with_input!(day15,"3,1,2");
//  one_day!(day15);


    Ok(())  
}
