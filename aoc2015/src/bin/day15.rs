use std::{
    fs,
    ops::{Add, Mul},
    usize,
};

#[derive(Default, Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn parse(input: &str) -> Ingredient {
        let mut parts = input.split_whitespace();
        parts.next();
        let mut get_next = || parts.nth(1).unwrap().trim_matches(',').parse().unwrap();
        Ingredient {
            capacity: get_next(),
            durability: get_next(),
            flavor: get_next(),
            texture: get_next(),
            calories: get_next(),
        }
    }

    fn score(&self) -> i64 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}

impl Mul<usize> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self {
            capacity: self.capacity * rhs as i64,
            durability: self.durability * rhs as i64,
            flavor: self.flavor * rhs as i64,
            texture: self.texture * rhs as i64,
            calories: self.calories * rhs as i64,
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

fn score(
    ingreds: &[Ingredient],
    idx: usize,
    used: usize,
    prev: Ingredient,
    check_calory: bool,
) -> i64 {
    if idx == ingreds.len() - 1 {
        let cookie = ingreds[idx] * (100 - used) + prev;
        if check_calory && cookie.calories != 500 {
            0
        } else {
            cookie.score()
        }
    } else {
        let mut max = 0;
        for spoons in 0..=100 - used {
            max = max.max(score(
                ingreds,
                idx + 1,
                used + spoons,
                prev + ingreds[idx] * spoons,
                check_calory,
            ));
        }
        max
    }
}

fn main() {
    let ingreds = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Ingredient::parse)
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        score(&ingreds, 0, 0, Ingredient::default(), false)
    );
    println!(
        "Part 2: {}",
        score(&ingreds, 0, 0, Ingredient::default(), true)
    );
}
