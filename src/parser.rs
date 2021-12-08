
pub fn one_int_per(input:&str,p:char)->Vec<i64>{
    input.split(p)
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i64>().ok())
                     .collect()
}

pub fn one_int_match<F:FnMut(char) -> bool>(input:&str,p:F)->Vec<i64>{
    input.split(p)
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i64>().ok())
                     .collect()
}

pub fn coma_separated_int(input:&str) -> Vec<i64>{
    one_int_per(input, ',')
}

#[allow(dead_code)]
pub fn one_string_per_line(input:& str) -> Vec<String>{
    input.split('\n').map(str::trim)
                     .filter(|x| !x.is_empty())
                     .map(|x| x.to_string())
                     .collect()
}

pub fn one_x_per_y<T,F>(input:& str,separator:char,line_parser:F)->Vec<T>
where
F: Fn(&str)->T
{
    input.split(separator).map(str::trim)
                          .filter(|x| !x.is_empty())
                          .map(|x| line_parser(x))
                          .collect()
}

#[allow(non_snake_case)]
pub fn one__per_line<T,F>(input:& str,line_parser:F) -> Vec<T>
where
F: Fn(&str)->T
{
    input.split('\n').map(str::trim)
                     .filter(|x| !x.is_empty())
                     .map(|x| line_parser(x))
                     .collect()
}

pub fn one_int_per_line(input:& str)->Vec<i64>{
    one_int_per(input, '\n')
}
pub fn one_char_vec_per_line(input:& str)->Vec<Vec<char>>{
    input.split('\n').map(str::trim).filter(|x| !x.is_empty())
                     .map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn items_separated_by_whitespace_separated_by_blankline(input:& str)->Vec<Vec<String>>{
    input.split("\n\n").map(str::trim).filter(|x| !x.is_empty())
                        .map(|l| l.split(char::is_whitespace).filter(|x| !x.is_empty()).map(|x|x.to_string()).collect()).collect()
}
pub fn separated_by_blank(input:& str)-> impl Iterator<Item=&str>{
    input.split("\n\n").map(str::trim).filter(|x|{!x.is_empty()})
}

pub fn bingo(input:&str)->(Vec<i64>,Vec<Vec<Vec<i64>>>){
    let mut input = separated_by_blank(input);
    let numbers = coma_separated_int(input.next().unwrap());
    let bingo = input.map(|x|{one__per_line(x,|y|{one_int_per(y, ' ')})}).collect();
    (numbers, bingo)
}

pub fn int_separated_by_any(input:&str)->Vec<i64>{
    one_int_match(input,|x| {!x.is_ascii_digit()})
}