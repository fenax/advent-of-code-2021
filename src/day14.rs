use std::collections::HashMap;

use itertools::Itertools;

use crate::formater::*;

pub const FILE: usize = 14;
type Int = i32;
type Data = (Vec<u8>, HashMap<(u8, u8), u8>);

struct PolymerIter<'a, It>
where
    It: Iterator<Item = u8>,
{
    iter: &'a mut It,
    map: &'a HashMap<(u8, u8), u8>,
    last: Option<u8>,
    next: Option<u8>,
}

impl<'a, It> Iterator for PolymerIter<'a, It>
where
    It: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if let Some(i) = self.next {
            let out = i;
            self.next = None;
            self.last = Some(out);
            Some(out)
        } else {
            if let Some(new) = self.iter.next() {
                if let Some(last) = self.last {
                    if let Some(insert) = self.map.get(&(last, new)) {
                        self.next = Some(new);
                        Some(*insert)
                    } else {
                        self.last = Some(new);
                        Some(new)
                    }
                } else {
                    self.last = Some(new);
                    Some(new)
                }
            } else {
                None
            }
        }
    }
}

fn apply_polymer<'a, It>(iter: &'a mut It, map: &'a HashMap<(u8, u8), u8>) -> PolymerIter<'a, It>
where
    It: Iterator<Item = u8>,
{
    PolymerIter {
        iter,
        map,
        last: None,
        next: None,
    }
}

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let v: Vec<&str> = input.split("\n\n").collect();
    let mut map = HashMap::new();
    for l in v[1].lines() {
        let l = l.as_bytes();
        map.insert((l[0] - b'A', l[1] - b'A'), l[6] - b'A');
    }
    (v[0].bytes().map(|x| x - b'A').collect(), map)
}

fn part_1(data: &Data) -> Int {
    let (input, map) = data;
    let value = (0..10).fold(input.clone(), |input, i| {
        let mut iter = input.iter().cloned();
        let out = apply_polymer(&mut iter, map).collect();
        //println!("{:?}", out);
        out
    });
    let step = value.iter();
    /*let mut iter = input.iter().cloned();

    let mut step = apply_polymer(&mut iter, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);
    let mut step = apply_polymer(&mut step, map);*/
    let mut count = [0i32; 26];
    step.for_each(|i| count[*i as usize] += 1);
    // println!("{:?}", count);
    let mut max = 0;
    //let max_id = 0;
    let mut min = i32::MAX;
    //let min_it = 0;
    for &x in count.iter() {
        if x > max {
            max = x;
        }
        if x < min && x > 0 {
            min = x;
        }
    }
    //println!("{} {}", min, max);
    max - min
}

fn part_2(data: &Data) -> i64 {
    fn one_cycle(
        pairs: &HashMap<(u8, u8), i64>,
        map: &HashMap<(u8, u8), u8>,
    ) -> HashMap<(u8, u8), i64> {
        let mut out = HashMap::new();
        for ((a, b), count) in pairs.iter() {
            if let Some(c) = map.get(&(*a, *b)) {
                out.entry((*a, *c))
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
                out.entry((*c, *b))
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
            }
        }
        out
    }
    let mut pairs = HashMap::new();
    for x in data.0.iter().cloned().tuple_windows::<(u8, u8)>() {
        pairs.entry(x).and_modify(|v| *v += 1).or_insert(1);
    }

    for i in 0..40 {
        let out = one_cycle(&pairs, &data.1);
        pairs = out;
    }

    let mut count_table = [0i64; 26];

    for ((a, b), count) in pairs.iter() {
        count_table[*b as usize] += *count;
    }

    count_table[data.0[0] as usize] += 1;

    //step.for_each(|i| count[i as usize] += 1);
    println!("{:?}", count_table);
    let mut max = 0i64;
    //let max_id = 0;
    let mut min = i64::MAX;
    //let min_it = 0;
    for &x in count_table.iter() {
        if x > max {
            max = x;
        }
        if x < min && x > 0 {
            min = x;
        }
    }
    //println!("{} {}", min, max);
    max - min
}

#[cfg(test)]
mod tests {
    use crate::day14::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);

        assert_eq!(part_1(&exemple), 1588);
        assert_eq!(part_2(&exemple), 2188189693529i64);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 2988));
        assert_eq!(part_2(&data).to_string(), format!("{}", 0));
    }
}
