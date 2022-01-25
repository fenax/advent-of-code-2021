use crate::formater::*;
use crate::parser;
use colored::*;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(11,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day11::{parse,part1,part2,step};

    static EXEMPLE:&'static str = 
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    static EXEMPLE2:&'static str =
r#"11111
19991
19191
19991
11111"#;
static RESULT2:&'static str =
r#"45654
51115
61116
51115
45654"#;
    #[test]
    fn test_small(){
        let mut data = parse(EXEMPLE2);
        let result = parse(RESULT2);
        let mut count = 0;
        step(&mut data,&mut count);
        step(&mut data,&mut count);
        assert_eq!(data,result);

    }

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",1656));
        assert_eq!(part2(&data), format!("solution {}",195));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(11));
        assert_eq!(part1(&data), format!("solution {}",1729));
        assert_eq!(part2(&data), format!("solution {}",237));
    }
}

fn parse(input:&str)->Vec<Vec<u32>>{
    parser::one_int_vec_per_line(input)
}


fn part2(data:&Vec<Vec<u32>>)->String{
    let mut data:Vec<Vec<u32>> = data.clone();
    let mut count:u32 = 0;

    let mut solution = 0; 
    for i in 0..1000{
        step(&mut data, &mut count);
        if data.iter().map(|x|{x.iter().sum::<u32>()}).sum::<u32>() == 0 as u32{
            solution = i+1;
            break;
        }
    }

    format!("solution {}",solution)
}

fn step(data:&mut Vec<Vec<u32>>,count:&mut u32){
    for line in data.iter_mut(){
        for x in line{
            *x+=1;
        }
    }

    for y in 0..data.len(){
        for x in 0..data[0].len(){
            if data[y][x] == 10 {propagate(data, x, y, count)}
        }
    }

    for line in data.iter_mut(){
        for x in line{
            if *x >= 10 {
                *x = 0;
//                print!("{}","0".bold());
            }else{
//                print!("{}",x);
            }
        }
//        print!("\n");
    }  
//    print!("\n");  
}

fn part1(data:&Vec<Vec<u32>>)->String{
    let mut data:Vec<Vec<u32>> = data.clone();
    let mut count:u32 = 0;

    for i in 0..100{
        step(&mut data, &mut count);
    }

    format!("solution {}",count)
}

fn propagate(data:&mut Vec<Vec<u32>>,x:usize,y:usize,count:&mut u32){
    if data[y][x]>=20 {return;}
    data[y][x]=20;
//    println!("prop {} {}",y,x);
    let h = data.len();
    let w = data[0].len();
    *count += 1;
    if y>0 {
        if x>0 {
            data[y-1][x-1] += 1;
            if data[y-1][x-1] >= 10 {
                propagate(data,x-1,y-1,count);
            }
        }
        data[y-1][x] += 1;
        if data[y-1][x] >= 10{
            propagate(data, x, y-1,count);
        }
        if x<w-1 {
            data[y-1][x+1] += 1;
            if data[y-1][x+1] >= 10 {
                propagate(data,x+1,y-1,count);
            }                
        }
    }
    if x>0 {
        data[y][x-1] += 1;
        if data[y][x-1] >= 10 {
            propagate(data,x-1,y,count);
        }
    }
    if x<w-1 {
        data[y][x+1] += 1;
        if data[y][x+1] >= 10 {
            propagate(data,x+1,y,count);
        }
    }
    if y<h-1 {
        if x>0 {
            data[y+1][x-1] += 1;
            if data[y+1][x-1] >= 10 {
                propagate(data,x-1,y+1,count);
            }
        }
        data[y+1][x] += 1;
        if data[y+1][x] >= 10 {
            propagate(data,x,y+1,count);
        }
        if x<w-1 {
            data[y+1][x+1] += 1;
            if data[y+1][x+1] >= 10 {
                propagate(data,x+1,y+1,count);
            }
        }            
    }
}
