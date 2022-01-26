use im::*;

use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(12,
        parse,
        part1,
        part2
    )
}

#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day12::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    static EXEMPLE2:&'static str = 
r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    static  EXEMPLE3:&'static str =
r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;


    #[test]
    fn test_zero(){
        let data = parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",10));
        assert_eq!(part2(&data), format!("solution {}",36));
    }

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE2);
        assert_eq!(part1(&data), format!("solution {}",19));
        assert_eq!(part2(&data), format!("solution {}",103));
    }

    #[test]
    fn test_example3() {
        let data =  parse(EXEMPLE3);
        assert_eq!(part1(&data), format!("solution {}",226));
        assert_eq!(part2(&data), format!("solution {}",3509));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(12));
        assert_eq!(part1(&data), format!("solution {}",354564));
        assert_eq!(part2(&data), format!("solution {}",1609058859115i64));
    }
}

fn parse(input:&str)->Vec<Vec<String>>{
    let input:Vec<Vec<String>> = 
        parser::one_x_per_y(input,'\n',|x|{x.split('-').map(str::trim).map(str::to_string).collect()});
    input
}

fn recurse_second(stack :&mut Vec<String>, data: &HashMap<String,Vec<String>>, list: &mut Vec<Vec<String>>,small:bool){
    let last = stack.last().unwrap();
    if last == "end"{
//        println!("{:?} {}",stack,small);
        list.push(stack.clone());
    }else{
        for i in data.get(last).unwrap(){
            let mut small = small;

            if i.chars().next().unwrap().is_lowercase(){
                if stack.contains(i){
                    if small{
                        continue;
                    }else{
                        small = true;
                    }
                }
            }
            
            stack.push(i.clone());
            recurse_second(stack, data, list,small);
        }
    }
    stack.pop();
}

fn part2(input:&Vec<Vec<String>>)->String{

    let mut data = HashMap::new();
    for i in input{
        if i[1] != "start"{
            data.entry(i[0].clone()).or_insert(Vec::new()).push(i[1].clone());
        }
        if i[0] != "start"{
            data.entry(i[1].clone()).or_insert(Vec::new()).push(i[0].clone());
        }
    }
    

    let mut path = Vec::new();
    let mut list = Vec::new();
    path.push("start".to_string());

    recurse_second(&mut path, &data, &mut list,false);

    let solution = list.len();
    format!("solution {}",solution)
}
/* 
fn recurse(stack :&mut Vec<String>, data: &HashMap<String,Vec<String>>, list: &mut Vec<Vec<String>>){
    let last = stack.last().unwrap();
    if last == "end"{
        list.push(stack.clone());
    }else{
        for i in data.get(last).unwrap(){
            if i.chars().next().unwrap().is_lowercase(){
                if stack.contains(i){
                    continue;
                }
            }
            
            stack.push(i.clone());
            recurse(stack, data, list);
        }
    }
    stack.pop();
} */

fn part1(input:&Vec<Vec<String>>)->String{
    let mut data = HashMap::new();
    for i in input{
        data.entry(i[0].clone()).or_insert(Vec::new()).push(i[1].clone());
        data.entry(i[1].clone()).or_insert(Vec::new()).push(i[0].clone());
    }
    

    let mut path = Vec::new();
    let mut list = Vec::new();
    path.push("start".to_string());

    recurse_second(&mut path, &data, &mut list,true);
    let solution = list.len();
    format!("solution {}",solution)
}
