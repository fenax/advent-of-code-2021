use crate::formater::*;
use crate::parser;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(13,
        parse,
        part1,
        part2
    )
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day13::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

static RESULT:&'static str =
r#"#####
#...#
#...#
#...#
#####
"#;

static SOLUTION:&'static str =
r#"###...##..####.###...##..####..##..###.
#..#.#..#....#.#..#.#..#.#....#..#.#..#
###..#......#..#..#.#....###..#..#.###.
#..#.#.....#...###..#....#....####.#..#
#..#.#..#.#....#.#..#..#.#....#..#.#..#
###...##..####.#..#..##..####.#..#.###.
"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",17));
        assert_eq!(part2(&data), format!("solution \n{}",RESULT));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(13));
        assert_eq!(part1(&data), format!("solution {}",847));
        assert_eq!(part2(&data), format!("solution \n{}",SOLUTION));
    }
}

fn parse(input:&str)->(Vec<(u32,u32)>,Vec<(char,u32)>){
    let mut i = input.split("\n\n");
    let dots = i.next().unwrap();
    let folds = i.next().unwrap();

    (parser::one_x_per_y(dots,'\n',
        |l|{
            let mut l = l.split(',');
            (l.next().unwrap().parse::<u32>().unwrap(), l.next().unwrap().parse::<u32>().unwrap())
        }),
     parser::one_x_per_y(folds,'\n',
        |l|{
            let mut l = l.split('=');
            (l.next().unwrap().chars().last().unwrap(),l.next().unwrap().parse::<u32>().unwrap())
        }))
}

fn part2((data,folds):&(Vec<(u32,u32)>,Vec<(char,u32)>))->String{
    let new = fold(data,folds);

    let (xmax,ymax) = new.iter().fold((0,0),|(xmax,ymax),(x,y)|{
        (xmax.max(*x),ymax.max(*y))
    });

    let mut out: Vec<char> = vec!['.';((xmax+2)*(ymax+1))as usize];

    for i in 1..=ymax+1{
        out[(i*(xmax+2)-1) as usize] = '\n';
    }
    for (x,y) in new{
        out[(x+(y*(xmax+2))) as usize]='#';
    }

    let solution :String = out.iter().collect();
    format!("solution \n{}",solution)
}

fn insert_ord<T:Ord>(v:&mut Vec<T>,i: T){
    match v.binary_search(&i){
        Ok(_pos) => {},
        Err(pos) => v.insert(pos, i),
    }
}

fn fold(data:&Vec<(u32,u32)>,folds:&Vec<(char,u32)>)->Vec<(u32,u32)>{
    let mut current:Vec<(u32,u32)> = Vec::new();

    for i in data{
        insert_ord(&mut current,*i);
    }
    
    for (axis,fold) in folds{
        let mut new:Vec<(u32,u32)> = Vec::new();
        for (x,y) in current{
            insert_ord(&mut new,
            if *axis == 'x'{
                if x < *fold{
                    (x,y)
                }else{
                    (*fold-(x-*fold) ,y)
                }
            }else{
                if y < *fold{
                    (x,y)
                }else{
                    (x,*fold-(y-*fold))
                }
            });
        }
        current = new;
    }

    current
}

fn part1((data,folds):&(Vec<(u32,u32)>,Vec<(char,u32)>))->String{
    let mut v = Vec::new();
    v.push(folds[0]);
    let new = fold(data,&v);


    let solution = new.len();
    format!("solution {}",solution)
}
