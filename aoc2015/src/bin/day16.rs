use std::{collections::HashMap, fs};

fn cond1(_: &str, v: u32, guess: u32) -> bool {
    v == guess
}

fn cond2(k: &str, v: u32, guess: u32) -> bool {
    match k {
        "cats" | "trees" => v > guess,
        "pomeranians" | "goldfish" => v < guess,
        _ => v == guess,
    }
}

fn get_matches(
    sue: &HashMap<&str, u32>,
    guess: &HashMap<&str, u32>,
    cond: impl Fn(&str, u32, u32) -> bool,
) -> u32 {
    sue.iter().map(|(&k, &v)| cond(k, v, guess[k]) as u32).sum()
}

fn main() {
    let rgx = regex::Regex::new(r#"(\w+: \d+)"#).unwrap();

    let mut guess = HashMap::<&str, u32>::new();
    guess.insert("children", 3);
    guess.insert("cats", 7);
    guess.insert("samoyeds", 2);
    guess.insert("pomeranians", 3);
    guess.insert("akitas", 0);
    guess.insert("vizslas", 0);
    guess.insert("goldfish", 5);
    guess.insert("trees", 3);
    guess.insert("cars", 2);
    guess.insert("perfumes", 1);
    let guess = guess;

    let input = fs::read_to_string("input.txt").unwrap();

    let sues = input
        .lines()
        .map(|l| {
            rgx.captures_iter(l)
                .map(|m| {
                    let mut splits = m.get(0).unwrap().as_str().split(": ");
                    (
                        splits.next().unwrap(),
                        splits.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<HashMap<&str, u32>>()
        })
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        sues.iter()
            .position(|s| get_matches(s, &guess, cond1) == 3)
            .unwrap()
            + 1
    );

    println!(
        "Part 2: {}",
        sues.iter()
            .position(|s| get_matches(s, &guess, cond2) == 3)
            .unwrap()
            + 1
    );
}
