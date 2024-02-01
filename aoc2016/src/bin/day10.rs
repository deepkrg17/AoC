use std::{collections::HashMap, fs};

enum Target {
    Bot(usize),
    Output(usize),
}

impl Target {
    fn new(target: &str, value: &str) -> Self {
        let id = value.parse::<usize>().unwrap();
        match target {
            "bot" => Target::Bot(id),
            "output" => Target::Output(id),
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    bot: usize,
    low: Target,
    high: Target,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    let mut instrucs = Vec::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let mut nth = |n| split.nth(n).unwrap();
        if nth(0) == "value" {
            let value = nth(0).parse::<usize>().unwrap();
            bots.entry(nth(3).parse::<usize>().unwrap())
                .or_insert_with(Vec::new)
                .push(value);
        } else {
            instrucs.push(Instruction {
                bot: nth(0).parse::<usize>().unwrap(),
                low: Target::new(nth(3), nth(0)),
                high: Target::new(nth(3), nth(0)),
            });
        }
    });

    while bots.values().map(Vec::len).sum::<usize>() > 0 {
        for Instruction { bot, low, high } in &instrucs {
            let vals = bots.entry(*bot).or_insert(Vec::new());
            if vals.len() < 2 {
                continue;
            }
            vals.sort();
            let highv = vals.pop().unwrap();
            let lowv = vals.pop().unwrap();
            if lowv == 17 && highv == 61 {
                println!("Part 1: {}", bot);
            }
            for (v, t) in [(lowv, low), (highv, high)] {
                match t {
                    Target::Bot(id) => bots.entry(*id).or_insert(Vec::new()).push(v),
                    Target::Output(id) => {
                        outputs.insert(*id, v);
                    }
                }
            }
        }
    }

    println!(
        "Part 2: {}",
        (0..=2).map(|i| outputs[&i]).product::<usize>()
    );
}
