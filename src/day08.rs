use std::fmt::Debug;

use crate::formater::*;
//use crate::parser;
use crate::parser::one_x_per_y;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(8,
        parse,
        part1,
        part2
    )
}

#[derive(Clone, Copy,Debug,PartialEq, Eq)]
struct Disp{
    a:bool,
    b:bool,
    c:bool,
    d:bool,
    e:bool,
    f:bool,
    g:bool
}
const NUMBERS:[Disp;10] = [Disp{a:true ,b:true ,c:true ,d:false,e:true ,f:true ,g:true },
                           Disp{a:false,b:false,c:true ,d:false,e:false,f:true ,g:false},
                           Disp{a:true ,b:false,c:true ,d:true ,e:true ,f:false,g:true },
                           Disp{a:true ,b:false,c:true ,d:true ,e:false,f:true ,g:true },
                           Disp{a:false,b:true ,c:true ,d:true ,e:false,f:true ,g:false},
                           Disp{a:true ,b:true ,c:false,d:true ,e:false,f:true ,g:true },
                           Disp{a:true ,b:true ,c:false,d:true ,e:true ,f:true ,g:true },
                           Disp{a:true ,b:false,c:true ,d:false,e:false,f:true ,g:false},
                           Disp{a:true ,b:true ,c:true ,d:true ,e:true ,f:true ,g:true },
                           Disp{a:true ,b:true ,c:true ,d:true ,e:false,f:true ,g:true }];

impl Default for Disp{
    fn default() -> Self {
        Disp { a:false, b: false, c: false, d: false, e: false, f: false, g: false }
    }
}

impl std::ops::Not for Disp{
    type Output = Disp;
    fn not(self) -> Disp{
        Disp{a:!self.a,b:!self.b,c:!self.c,d:!self.d,e:!self.e,f:!self.f,g:!self.g}
    }
}
impl std::ops::BitAnd for Disp{
    type Output = Disp;
    fn bitand(self, rhs: Self) -> Self::Output {
        Disp{a:self.a&rhs.a,b:self.b&rhs.b,c:self.c&rhs.c,d:self.d&rhs.d,e:self.e&rhs.e,f:self.f&rhs.f,g:self.g&rhs.g}
    }
}
impl std::ops::BitOr for Disp{
    type Output = Disp;
    fn bitor(self, rhs: Self) -> Self::Output {
        Disp{a:self.a|rhs.a,b:self.b|rhs.b,c:self.c|rhs.c,d:self.d|rhs.d,e:self.e|rhs.e,f:self.f|rhs.f,g:self.g|rhs.g}
    }
}

impl Disp{
    fn set(&mut self,v: char){
        match v{
            'a'=>{self.a=true;},
            'b'=>{self.b=true;},
            'c'=>{self.c=true;},
            'd'=>{self.d=true;},
            'e'=>{self.e=true;},
            'f'=>{self.f=true;},
            'g'=>{self.g=true;},
            _ => panic!("invalid segment {}",v)
        }
    }
    fn build(&self,a:Disp,b:Disp,c:Disp,d:Disp,e:Disp,f:Disp,g:Disp)->Disp{
        let mut out = Self::default();
        if self.a{out=out|a}
        if self.b{out=out|b}
        if self.c{out=out|c}
        if self.d{out=out|d}
        if self.e{out=out|e}
        if self.f{out=out|f}
        if self.g{out=out|g}
        out
    }
    fn len(&self)->usize{
        let mut size:usize = 0;
        if self.a {size+=1}
        if self.b {size+=1}
        if self.c {size+=1}
        if self.d {size+=1}
        if self.e {size+=1}
        if self.f {size+=1}
        if self.g {size+=1}
        size
    }
    fn parse(input:&str)->Disp{
        let mut out = Self::default();
        for i in input.chars(){
            out.set(i);
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day08::{parse,part1,part2};

    static EXEMPLE:&'static str = 
r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_example() {
        let data =  parse(EXEMPLE);
        assert_eq!(part1(&data), format!("solution {}",26));
        assert_eq!(part2(&data), format!("solution {}",61229));
    }

    #[test]
    fn test(){
        let data = parse(&read_file(8));
        assert_eq!(part1(&data), format!("solution {}",539));
        assert_eq!(part2(&data), format!("solution {}",1084606));
    }
}

fn parse(input:&str)->Vec<Vec<Vec<Disp>>>{
    one_x_per_y(input, '\n', |l|{
        one_x_per_y(l, '|', |i|{
            one_x_per_y(i, ' ', Disp::parse)})})
}

fn build_truth_table(input:&Vec<Vec<Disp>>)->Vec<Disp>{
    let un = input[0].iter().find(|x|{x.len()==2}).unwrap().clone();
    let deuxtroiscinq:Vec<Disp> = input[0].iter().filter(|x|{x.len()==5}).cloned().collect();
    let quatre = input[0].iter().find(|x|{x.len()==4}).unwrap().clone();
    let zerosixneuf:Vec<Disp> = input[0].iter().filter(|x|{x.len()==6}).cloned().collect();
    let sept = input[0].iter().find(|x|{x.len()==3}).unwrap().clone();
//    let huit = input[0].iter().find(|x|{x.len()==7}).unwrap();

    let [mut a,mut b,mut c,mut d,mut e,mut f,mut g] = 
            [!Disp::default();7];

    let not_un = !un;
    c = un    &c;
    f = un    &f;
    a = not_un&a;
    b = not_un&b;
    d = not_un&d;
    e = not_un&e;
    g = not_un&g;

    let not_quatre = !quatre;
    b = quatre&b;
    c = quatre&c;
    d = quatre&d;
    f = quatre&f;
    a = not_quatre&a;
    e = not_quatre&e;
    g = not_quatre&g;

    let not_sept = !sept;
    a = sept&a;
    c = sept&c;
    f = sept&f;
    b = not_sept&b;
    d = not_sept&d;
    e = not_sept&e;
    g = not_sept&g;

    let fixe = deuxtroiscinq.iter().fold(!Disp::default(),|d,&i|{d&i});
    let vari = !fixe;
    a = fixe&a;
    d = fixe&d;
    g = fixe&g;
    b = vari&b;
    c = vari&c;
    e = vari&e;
    f = vari&f;

    let fixe = zerosixneuf.iter().fold(!Disp::default(),|d,&i|{d&i});
    let vari = !fixe;    
    
    a = fixe&a;
    b = fixe&b;
    f = fixe&f;
    g = fixe&g;
    c = vari&c;
    d = vari&d;
    e = vari&e;



    NUMBERS.iter().map(|x|{x.build(a, b, c, d, e, f, g)}).collect()
}

fn part2(data:&Vec<Vec<Vec<Disp>>>)->String{
    let mut solution = 0;
    for i in data{
        let table = build_truth_table(i);
        let mut num = 0;
        for j in &i[1]{
            num = num * 10 + 
            table.iter().position(|&x|*j==x).unwrap()
        }
        solution+=num;
    }
    format!("solution {}",solution)
}

fn part1(data:&Vec<Vec<Vec<Disp>>>)->String{
    let mut  solution = 0;
    
    for i in data{
        for j in &i[1]{
            match j.len(){
                2|3|4|7 =>{solution+=1;}
                _ => {}
            }
        }
    }

    format!("solution {}",solution)
}
