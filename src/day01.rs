use itertools::{izip, Itertools};
use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(1,
        parser::one_int_per_line,
        count_increases,
        part2)
}

fn count_increases(data:&Vec<i64>)->usize{
    let mut iter1 = data.iter();
    iter1.next();
    iter1.zip(data.iter()).filter(|(a,b)| {a>b}).count()
}

fn rolling_sum(data:&Vec<i64>)->Vec<i64>{
    let iter0 = data.iter();
    let mut iter1 = data.iter();
    let mut iter2 = data.iter();
    iter1.next();
    iter2.next();
    iter2.next();

    izip!(iter0,iter1,iter2).map(|(a,b,c)|{a+b+c}).collect_vec()
}

fn part2(data:&Vec<i64>)->usize{
    let sum = rolling_sum(data);
    count_increases(&sum)
}

#[cfg(test)]
mod tests {
    use crate::parser::one_int_per_line;
    use crate::formater::read_file;
    use crate::day01::{count_increases, part2};

    static EXEMPLE:&'static[i64] = &[199,200,208,210,200,207,240,269,260,263];

    #[test]
    fn test_example() {
        assert_eq!(count_increases(&EXEMPLE.to_vec()), 7);
        assert_eq!(part2(&EXEMPLE.to_vec()),5)
    }
    #[test]
    fn test(){
        let data = one_int_per_line(&read_file(1));
        assert_eq!(count_increases(&data).to_string(), format!("{}",1342));
        assert_eq!(part2(&data).to_string(), format!("{}",1378));
    }
}
