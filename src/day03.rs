use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(3,
        parser::one_char_vec_per_line,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::parser::one_char_vec_per_line;
    use crate::day03::{part1,part2};

    static EXEMPLE:&'static str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn part1_test() {
        let data =  one_char_vec_per_line(EXEMPLE);
        assert_eq!(part1(&data), format!("gamma {} [{}] epsilon {} [{}] solution {}","10110",22,"01001",9,198));
    }
    #[test]
    fn part2_test() {
        let data =  one_char_vec_per_line(EXEMPLE);
        assert_eq!(part2(&data), format!("oxygen {} [{}] CO2 {} [{}] solution {}","10111",23,"01010",10,230));
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

fn count_ones_at(data:&Vec<Vec<char>>,skip:usize)->usize{
    data.iter().fold(0,|tmp,item|{if item[skip] == '1'{tmp+1}else{tmp}})
}

fn filter_most(data:Vec<Vec<char>>,skip:usize)->Vec<char>{
    let len = data.len();
    let c = count_ones_at(&data, skip);
    let want = if  c*2 >= len{'1'}else{'0'};
    let ndata:Vec<Vec<char>> = data.into_iter().filter(|x| x[skip] == want).collect::<Vec<Vec<char>>>();
    if ndata.len() == 1 {ndata[0].clone()}else{filter_most(ndata, skip+1)}
}
fn filter_least(data:Vec<Vec<char>>,skip:usize)->Vec<char>{
    let len = data.len();
    let c = count_ones_at(&data, skip);
    let want = if  c*2 < len{'1'}else{'0'};
    let ndata:Vec<Vec<char>> = data.into_iter().filter(|x| x[skip] == want).collect::<Vec<Vec<char>>>();
    if ndata.len() == 1 {ndata[0].clone()}else{filter_least(ndata, skip+1)}
}

fn charvec_to_int(data:&Vec<char>)->i32{
    data.iter().fold(0,|v,i|v*2+if *i == '1' {1}else{0})
}

fn part2(data:&Vec<Vec<char>>)->String{
    let ndata = data.clone();
    let oxygen = filter_most(ndata, 0);
    let oxygen_dec = charvec_to_int(&oxygen);
    let ndata = data.clone();
    let co2 = filter_least(ndata,0);
    let co2_dec = charvec_to_int(&co2);
    format!("oxygen {} [{}] CO2 {} [{}] solution {}",oxygen.iter().collect::<String>(),oxygen_dec,co2.iter().collect::<String>(),co2_dec,oxygen_dec*co2_dec)
    
}

fn count_ones_and_zeros(data:&Vec<Vec<char>>)->Vec<usize>{
    let temp:Vec<usize> = data.first().unwrap().iter().map(|_|0).collect();
    data.iter().fold(temp,|tmp,item|{tmp.iter().zip(item.iter()).map(|(i,c)|if *c == '1' {i+1}else{*i}).collect()})
}

