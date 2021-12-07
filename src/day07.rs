use std::collections::HashMap;
use itertools::GroupingMap;

use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(7,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day07::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {} at position {}",37,2));
        assert_eq!(part2(&data), format!("solution {} at position {}",168,5));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(7));
        assert_eq!(part1(&data), format!("solution 355764 at position 339"));
        assert_eq!(part2(&data), format!("solution 99634572 at position 485"));
    }
}

fn parse(input:&str)->Vec<i64>{
    parser::coma_separated_int(input)
}

fn build_countings(data:&Vec<i64>)->Vec<(i16,i16)>{
    let len = *data.iter().max().unwrap() as usize + 1;
    let mut out = Vec::with_capacity(len);
    out.resize(len,0);

    for i in data{
        out[*i as usize] += 1;
    }
    let out: Vec<(i16,i16)> = out.iter().enumerate().filter_map(|(i,&x)|if x>0 {Some((i as i16,x))}else{None}).collect();

    out
}

fn triangular(x:i64)->i64{
    x*(x+1)/2
}

fn part2(data:&Vec<i64>)->String{
    let data = build_countings(data);

    let mut min_value = i64::MAX;
    let mut min_index = 0;
    for i in 0..data.len(){
        let cur:i64 = data.iter().map(|(j,v)|{triangular((*j as i64-i as i64).abs())* *v as i64}).sum();
        if cur < min_value{
            min_value = cur;
            min_index = i;
        }
    }

    let solution = min_value;
    format!("solution {} at position {}",solution,min_index)
}

fn part1(data:&Vec<i64>)->String{
    let data = build_countings(data);

    let mut min_value = i64::MAX;
    let mut min_index = 0;
    for i in 0..data.len(){
        let cur:i64 = data.iter().map(|(j,v)|{(*j as i64-i as i64).abs()* *v as i64}).sum();
        if cur < min_value{
            min_value = cur;
            min_index = i;
        }
    }

    let solution = min_value;
    format!("solution {} at position {}",solution,min_index)
}

fn part1_maybe_faster(data:&Vec<i64>)->String{
    let map = build_hash_countings(data);

    let map:Vec<(i16,i16)> = map.iter().map(|(&a,&b)|(a,b)).collect();

    let mut min_value = i64::MAX;
    let mut min_index = 0;

    for i in 0..data.len(){
        let cur:i64 = map.iter().map(|(j,v)|{(*j as i64-i as i64).abs()* *v as i64}).sum();
        if cur < min_value{
            min_value = cur;
            min_index = i;
        }
    }

    let solution = min_value;
    format!("solution {} at position {}",solution,min_index) 
}

fn build_hash_countings(data:&Vec<i64>)->HashMap<i16,i16>{
    let mut out = HashMap::new();
    for i in data{
        let x = out.entry(*i as i16).or_insert(0i16);
        *x+=1;
    }
    out
}
