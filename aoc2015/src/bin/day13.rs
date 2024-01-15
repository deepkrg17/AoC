use std::{
    collections::{HashMap, HashSet},
    fs,
};

use utils::get_permutations;

type Pairs = HashMap<Pair, i32>;

#[derive(Hash, Eq, PartialEq)]
struct Pair {
    prev: String,
    next: String,
}

impl Pair {
    fn happiness(&self, pairs: &Pairs) -> i32 {
        *pairs.get(self).unwrap()
            + *pairs
                .get(&Pair {
                    prev: self.next.clone(),
                    next: self.prev.clone(),
                })
                .unwrap()
    }
}

fn parse_to_pairs(s: &str, pairs: &mut Pairs, names: &mut HashSet<String>) {
    let split = s.split_whitespace().collect::<Vec<_>>();

    let prev = split.first().unwrap().to_string();
    let next = split.last().unwrap().strip_suffix('.').unwrap().to_string();
    let change = split[3].parse::<i32>().unwrap() * if split[2] == "gain" { 1 } else { -1 };

    names.insert(prev.clone());
    pairs.insert(Pair { prev, next }, change);
}

fn opt_happyness(pairs: &Pairs, names: Vec<String>) -> i32 {
    get_permutations(names).into_iter().fold(0, |happyness, p| {
        let mut hpyns = 0;
        for i in 1..p.len() {
            let prev = p[i - 1].clone();
            let next = p[i].clone();
            hpyns += Pair { prev, next }.happiness(pairs);
        }
        hpyns += Pair {
            prev: p.last().unwrap().clone(),
            next: p.first().unwrap().clone(),
        }
        .happiness(pairs);
        happyness.max(hpyns)
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut names = HashSet::new();

    let mut pairs = input.lines().fold(Pairs::new(), |mut pairs, line| {
        parse_to_pairs(line, &mut pairs, &mut names);
        pairs
    });
    let mut names = names.into_iter().collect::<Vec<_>>();

    println!("Part 1: {}", opt_happyness(&pairs, names.clone()));

    for name in names.iter() {
        pairs.insert(
            Pair {
                prev: "Me".to_string(),
                next: name.clone(),
            },
            0,
        );
        pairs.insert(
            Pair {
                prev: name.clone(),
                next: "Me".to_string(),
            },
            0,
        );
    }
    names.push("Me".to_string());

    println!("Part 2: {}", opt_happyness(&pairs, names));
}
