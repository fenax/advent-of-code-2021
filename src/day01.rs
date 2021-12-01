use itertools::{izip, Itertools};
use crate::*;

pub fn run(input:&str){
    print_single_parse(1,
        ||{parser::one_int_per_line(input)},
        |data|{count_increases(data)},
        |data|{part2(data)});
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
    use crate::day01::{count_increases, part2};

    static EXEMPLE:&'static[i64] = &[199,200,208,210,200,207,240,269,260,263];

    #[test]
    fn part1_test() {
        assert_eq!(count_increases(&EXEMPLE.to_vec()), 7);
    }
    #[test]
    fn part2_test(){
        assert_eq!(part2(&EXEMPLE.to_vec()),5)
    }
}
