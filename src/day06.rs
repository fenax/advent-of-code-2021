use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(6,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day06::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"3,4,3,1,2"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",5934));
        assert_eq!(part2(&data), format!("solution {}",26984457539i64));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(6));
        assert_eq!(part1(&data), format!("solution {}",354564));
        assert_eq!(part2(&data), format!("solution {}",1609058859115i64));
    }
}

fn parse(input:&str)->Vec<i64>{
    parser::coma_separated_int(input)
}

fn build_countings(data:&Vec<i64>)->[i64;9]{
    let mut out = [0 as i64;9];

    for i in data{
        out[*i as usize] += 1;
    }

    out
}

fn part2(data:&Vec<i64>)->String{
    let mut countings = build_countings(data);
    for _x in 0..256{
        let [a,b,c,d,e,f,g,h,i] = countings;
        countings = [b,c,d,e,f,g,h+a,i,a];
    }

    let solution:i64 = countings.iter().sum();
    format!("solution {}",solution)
}

fn part1(data:&Vec<i64>)->String{
    let mut countings = build_countings(data);
    for _x in 0..80{
        let [a,b,c,d,e,f,g,h,i] = countings;
        countings = [b,c,d,e,f,g,h+a,i,a];
    }

    let solution:i64 = countings.iter().sum();
    format!("solution {}",solution)
}
