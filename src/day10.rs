use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(10,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day10::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",26397));
        assert_eq!(part2(&data), format!("solution {}",288957));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(10));
        assert_eq!(part1(&data), format!("solution {}",354564));
        assert_eq!(part2(&data), format!("solution {}",1609058859115i64));
    }
}

fn parse(input:&str)->Vec<Vec<char>>{
    parser::one_char_vec_per_line(input)
}

enum Parsed{
    Good,
    Partial(i64),
    Corrupt(char),
}

fn complete_score(v:&char)->i64{
    match v{
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn parse_line(line:&Vec<char>)->Parsed{
    let mut stack:Vec<char> = Vec::new();

    for i in line{
        match i{
            '<' | '(' | '{' | '[' => stack.push(*i),
            '>' =>{
                if stack.pop() != Some('<') {return Parsed::Corrupt('>')}
            },
            ')' =>{
                if stack.pop() != Some('(') {return Parsed::Corrupt(')')}
            },
            '}' =>{
                if stack.pop() != Some('{') {return Parsed::Corrupt('}')}
            },
            ']' =>{
                if stack.pop() != Some('[') {return Parsed::Corrupt(']')}
            },
            x => {return Parsed::Corrupt(*x)}
        }
    }

    if stack.is_empty(){
        Parsed::Good
    }else{
        stack.reverse();
        Parsed::Partial(stack.iter().map(complete_score).fold(0,|v,i| v*5+i))
    }
}

fn score(v:Parsed)->i64{
    match v {
        Parsed::Corrupt(')') => 3,
        Parsed::Corrupt(']') => 57,
        Parsed::Corrupt('}') => 1197,
        Parsed::Corrupt('>') => 25137,
        _ => 0,
    }
}

fn part2(data:&Vec<Vec<char>>)->String{

    let mut tmp :Vec<i64> = data.iter().map(parse_line).filter_map(|v|match v{Parsed::Partial(v) => Some(v),_=>None}).collect();

    tmp.sort();




    let solution:i64 = tmp[(tmp.len()-1)/2];
    format!("solution {}",solution)
}



fn part1(data:&Vec<Vec<char>>)->String{
    let solution:i64 = data.iter().map(parse_line).map(score).sum();


    format!("solution {}",solution)
}
