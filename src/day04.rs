use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(4,
        parser::bingo,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::parser::bingo;
    use crate::day04::{part1,part2};

    static EXEMPLE:&'static str = 
r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_example() {
        let data =  bingo(EXEMPLE);
        assert_eq!(part1(&data), format!("winner {} with number {} with sum {} solution {}",3,24,188,4512));
        assert_eq!(part2(&data), format!("winner {} with number {} with sum {} solution {}",2,13,148,1924));
    }
    #[test]
    fn test() {
        let data =  bingo(&read_file(4));
        assert_eq!(part1(&data), "winner 64 with number 77 with sum 539 solution 41503");
        assert_eq!(part2(&data), "winner 14 with number 14 with sum 227 solution 3178");
    }
}

fn part1(data:&(Vec<i64>,Vec<Vec<Vec<i64>>>))->String{
    let mut index = Vec::new();
    let mut grids = Vec::new();
    let win:u8 = 0b11111;
    for _i in 0..100{
        index.push(Vec::new());
    }
    for _i in 0..data.1.len(){
        grids.push([0 as u8;5]);
    }
    for (w,i) in data.1.iter().enumerate(){
        for (y,j) in i.iter().enumerate(){
            for (x, cell) in j.iter().enumerate(){
                index[*cell as usize].push((w,x,y));
            }
        }
    }
    let mut result = None;
    'outer:
    for i in &data.0{
        for (g,x,y) in &index[*i as usize]{
            grids[*g][*y]|= 1<<x;
            if grids[*g].iter().any(|x|*x==win) || (grids[*g].iter().fold(win,|x,v|{x&v}) != 0){
                result = Some((g,i));
                break 'outer;
            }
        }
    }


    let (grid, number) = result.unwrap();
    let mut sum = 0;
    for (y,line) in data.1[*grid].iter().enumerate(){
        for (x, number) in line.iter().enumerate(){
            if (grids[*grid][y])&(1<<x)==0{
                sum += number
            }
        }
    }
    format!("winner {} with number {} with sum {} solution {}",grid+1,number,sum, sum*number)
}


fn part2(data:&(Vec<i64>,Vec<Vec<Vec<i64>>>))->String{
    let mut index = Vec::new();
    let mut grids = Vec::new();
    let mut won = Vec::new();
    let win:u8 = 0b11111;
    for _i in 0..100{
        index.push(Vec::new());
    }
    for _i in 0..data.1.len(){
        grids.push([0 as u8;5]);
        won.push(false);
    }
    for (w,i) in data.1.iter().enumerate(){
        for (y,j) in i.iter().enumerate(){
            for (x, cell) in j.iter().enumerate(){
                index[*cell as usize].push((w,x,y));
            }
        }
    }
    let mut result = None;
    let mut cycles = 0;
        'outer:
        for i in &data.0{
            for (g,x,y) in &index[*i as usize]{
                if !won[*g]{
                    grids[*g][*y]|= 1<<x;
                    if grids[*g].iter().any(|x|*x==win) || (grids[*g].iter().fold(win,|x,v|{x&v}) != 0){
                        won[*g] = true;
                        cycles+=1;
                        if cycles+1>grids.len(){
                            result = Some((g,i));
                            break 'outer;
                        }
                    }
                }
            }
        }


    let (grid, number) = result.unwrap();
    let mut sum = 0;
    for (y,line) in data.1[*grid].iter().enumerate(){
        for (x, number) in line.iter().enumerate(){
            if (grids[*grid][y])&(1<<x)==0{
                sum += number
            }
        }
    }
    format!("winner {} with number {} with sum {} solution {}",grid+1,number,sum, sum*number)
}