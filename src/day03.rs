use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(3,
        parser::one_char_vec_per_line,
        part1,
        part1
    )
}


#[cfg(test)]
mod tests {
    use crate::parser::one_char_vec_per_line;
    use crate::day03::{part1};

    static EXEMPLE:&'static str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn part1_test() {
        let data =  one_char_vec_per_line(EXEMPLE);
        assert_eq!(part1(&data), format!("gamma {} [{}] epsilon {} [{}] solution {}","10110",22,"01001",9,198));
    }

}

fn part1(data:&Vec<Vec<char>>)->String{
    let len = data.len();
    let mut gamma = String::with_capacity(len);
    let mut epsilon = String::with_capacity(len);
    let (gamma_dec,epsilon_dec) = 
    count_ones_and_zeros(data).iter().fold(
        (0,0),
        |(gamma_dec,epsilon_dec),item|{
            if item*2>len {gamma.push('1');epsilon.push('0');(gamma_dec*2+1,epsilon_dec*2)}
            else {gamma.push('0');epsilon.push('1');(gamma_dec*2,epsilon_dec*2+1)}
        });
        format!("gamma {} [{}] epsilon {} [{}] solution {}",gamma,gamma_dec,epsilon,epsilon_dec,gamma_dec*epsilon_dec)
}

fn count_ones_and_zeros(data:&Vec<Vec<char>>)->Vec<usize>{
    let temp:Vec<usize> = data.first().unwrap().iter().map(|_|0).collect();
    data.iter().fold(temp,|tmp,item|{tmp.iter().zip(item.iter()).map(|(i,c)|if *c == '1' {i+1}else{*i}).collect()})
}

