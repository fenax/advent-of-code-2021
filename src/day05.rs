use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(5,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::day05::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn part1_test() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",5));
    }
    #[test]
    fn part2_test() {
        let data =  parse(EXEMPLE);
        assert_eq!(part2(&data), format!("solution {}",12));
    }
}

fn parse(input:&str)->Vec<Vec<i64>>{
    parser::one__per_line(input,parser::int_separated_by_any)
}

fn min_max<T:Ord+Copy>(a:T,b:T)->(T,T){
    (a.min(b),a.max(b))
}

fn intersect(a:&Vec<i64>,b:&Vec<i64>)->bool{
    let ax = min_max(a[0], a[2]);
    let ay = min_max(a[1], a[3]);

    let bx = min_max(b[0], b[2]);
    let by = min_max(b[1], b[3]);

    (ax.0 <= bx.1 || bx.0 <= ax.1) && (ay.0 <= by.1 || by.0 <= ay.1)
}

fn part1_bad(data:&Vec<Vec<i64>>)->String{
    let filtered:Vec<Vec<i64>> = data.iter().filter(|x|{x[0]==x[2]||x[1]==x[3]}).cloned().collect();
    let total = filtered.iter().enumerate().filter(|(i,v)|{filtered.iter().skip(i+1).any(|x|{intersect(v,x)}) }).count();
        format!("solution {}",total)
}

fn build_map(data:&Vec<Vec<i64>>)->[[u8;1000];1000]{
    let mut map = [[0 as u8;1000];1000];

    for v in data{
        let x_inc = (v[2]-v[0]);
        let y_inc = (v[3]-v[1]);
        let mut x = v[0];
        let mut y = v[1];
        for j in 0..=x_inc.abs().max(y_inc.abs()){
            map[x as usize][y as usize] += 1;
            x += x_inc.signum();
            y += y_inc.signum();
        }
    }
/*     let mut map = [[0 as u8;1000];1000];

    for x in filtered{
        if(x[1]==x[3]){
            let (min,max) = min_max(x[0], x[2]);
            for j in min..=max{
                map[x[1] as usize][j as usize]+=1;
            }
        }else if x[0]==x[2]  {
            let (min, max) = min_max(x[1], x[3]);
            for j in min..=max{
                map[j as usize][x[0] as usize]+=1;
            }
        }
    }
*/
    map
}

fn part2(data:&Vec<Vec<i64>>)->String{

    let map = build_map(data);
    let solution :usize =map.iter().map(|x|{x.iter().filter(|v|{**v>=2}).count()}).sum();

    format!("solution {}",solution)
}

fn part1(data:&Vec<Vec<i64>>)->String{
    let filtered:Vec<Vec<i64>> = data.iter().filter(|x|{x[0]==x[2]||x[1]==x[3]}).cloned().collect();

    let map = build_map(&filtered);
    let solution :usize =map.iter().map(|x|{x.iter().filter(|v|{**v>=2}).count()}).sum();

    format!("solution {}",solution)
}
