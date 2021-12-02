use itertools::{izip, Itertools};
use regex::internal::Input;
use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(2,
        parser,
        follow_path,
        follow_correct_path
    )
}

fn parser(input:&str)->Vec<(char,u32)>{
    parser::one__per_line(input,
            |line:&str|-> (char, u32){ 
                let mut c = line.chars(); 
                (c.next().expect("no first character"),
                 c.last().expect("no character in line").to_digit(10).unwrap())})
}

fn follow_path(data:&Vec<(char,u32)>)->String{
    let (x,y) = 
    data.iter().fold((0,0), |(x,y),(i,j)|{
        match i{
            'f' => (x+j,y),
            'u' => (x,y-j),
            'd' => (x,y+j),
            _ => panic!("impossible order")
        }
    });
    format!("distance: {} depth: {} solution: {}",x,y,x*y)
}

fn follow_correct_path(data:&Vec<(char,u32)>)->String{
    let (x,y,a) = 
    data.iter().fold((0,0,0), |(x,y,a),(i,j)|{
        match i{
            'f' => (x+j,y+a*j,a),
            'u' => (x,y,a-j),
            'd' => (x,y,a+j),
            _ => panic!("impossible order")
        }
    });
    format!("distance: {} depth: {} solution: {}",x,y,x*y)
}

#[cfg(test)]
mod tests {
    use crate::day02::{parser,follow_path,follow_correct_path};

    static EXEMPLE:&'static str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn part1_test() {
        let data =  parser(EXEMPLE);
        assert_eq!(follow_path(&data), format!("distance: {} depth: {} solution: {}",15,10,150));
    }
    #[test]
    fn part2_test() {
        let data =  parser(EXEMPLE);
        assert_eq!(follow_correct_path(&data), format!("distance: {} depth: {} solution: {}",15,60,900));
    }
}
