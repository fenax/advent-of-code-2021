use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::formater::*;
use crate::parser;
use colored::*;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(9,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day09::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {:?} {}",[1,0,5,5].to_vec(),15));
        assert_eq!(part2(&data), format!("solution {}",26984457539i64));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(9));
        assert_eq!(part1(&data), format!("solution {}",354564));
        assert_eq!(part2(&data), format!("solution {}",1609058859115i64));
    }
}

fn parse(input:&str)->Vec<Vec<i8>>{
    let tmp = parser::one_char_vec_per_line(input);
    let mut out = Vec::new();

    let mut line = Vec::with_capacity(tmp[0].len()+2);
    line.resize(tmp[0].len()+2,9);

    out.push(line.clone());
    for l in tmp{
        let mut nl = Vec::new();
        nl.push(9);
        for c in l{
            nl.push(c.to_digit(10).unwrap() as i8);
        }
        nl.push(9);
        out.push(nl);
    }
    out.push(line);
    out
}

fn part2(data:&Vec<Vec<i8>>)->String{

    let mut bassins:Vec<u16> = Vec::new();
    let mut join:Vec<(u16,u16)> = Vec::new();
    let mut count:HashMap<u16,u16> = HashMap::new();

    let stride = data[0].len();
    let mut last_bassin =0;
//    print!("\n");
    for y in 0..data.len(){
        for x in 0..stride{
//            let mut cur = data[y][x].to_string().normal();
            if data[y][x] == 9{
//                cur = cur.red();
                bassins.push(0);
            }else{
                let a = *bassins.last().unwrap_or(&0);
                let b =
                if bassins.len()>=stride{
                    bassins[bassins.len()-stride]
                }else{
                    0
                };
                if a == b {
                    if a == 0{
//                        cur = cur.yellow();
                        last_bassin += 1;
                        count.insert(last_bassin, 1);
                        bassins.push(last_bassin);
                    }else{
//                        cur = cur.green();
                        *(count.entry(a).or_default()) +=1;
                        count.entry(b);
                        bassins.push(b);
                    }

                }else{
                    if !join.ends_with(&[(a,b)]) && a != 0 && b != 0{
//                        cur = cur.blue();
                        join.push((a,b));
                        let b_val = *count.entry(b).or_default();
                        *(count.entry(a).or_default()) += b_val;
                        *(count.entry(b).or_default()) =0;
                    }
                    let sel = if a == 0 {b}else{a};
                    bassins.push(sel);
                    *(count.entry(sel).or_default()) +=1;
                }
            }
//            print!("{}",cur);
        }
//        print!("\n");
    }

    let mut top:Vec<(u16,u16)> = count.iter().filter_map(|(k,v)|{if k>&0 {Some((*v,*k))} else {None}}).collect();

    top.sort();
    top.reverse();

    let solution:i64 = top[0].0 as i64 * top[1].0 as i64 * top[2].0 as i64;
    format!("solution {:?} {:?} {}",count, top,solution)
}

fn is_low(data:&Vec<Vec<i8>>,x:usize,y:usize)->bool{
    let i = data[y][x];
    i < data[y-1][x] &&
    i < data[y+1][x] &&
    i < data[y][x-1] &&
    i < data[y][x+1]
}

fn part1(data:&Vec<Vec<i8>>)->String{

    let mut list:Vec<i8> = Vec::new();

    for y in (1..data.len()-1){
        for x in (1..data[0].len()-1){
            if is_low(data, x,y){
                list.push(data[y][x]);
            }
        }
    }

    let solution:i64 = list.iter().fold(0 as i64,|v,i| v+((*i)as i64 +1));
    format!("solution {:?} {}",list,solution)
}
