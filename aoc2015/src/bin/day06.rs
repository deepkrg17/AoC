use regex::Regex;
use std::fs;

type Grid = [[u32; 1000]; 1000];
type Pos = (usize, usize);

fn turn_on(g: &mut Grid, b: &mut Grid, p1: Pos, p2: Pos) {
    for y in p1.0..=p2.0 {
        for x in p1.1..=p2.1 {
            g[y][x] = 1;
            b[y][x] += 1;
        }
    }
}

fn toggle(g: &mut Grid, b: &mut Grid, p1: Pos, p2: Pos) {
    for y in p1.0..=p2.0 {
        for x in p1.1..=p2.1 {
            g[y][x] ^= 1;
            b[y][x] += 2;
        }
    }
}

fn turn_off(g: &mut Grid, b: &mut Grid, p1: Pos, p2: Pos) {
    for y in p1.0..=p2.0 {
        for x in p1.1..=p2.1 {
            g[y][x] = 0;
            if b[y][x] > 0 {
                b[y][x] -= 1;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut g: Grid = [[0; 1000]; 1000];
    let mut b: Grid = [[0; 1000]; 1000];

    let pattern = Regex::new(r#"^([\w\s]+) (\d+),(\d+) through (\d+),(\d+)$"#).unwrap();

    input.lines().for_each(|line| {
        let c = pattern.captures(line).unwrap();
        let f = match c.get(1).unwrap().as_str() {
            "turn on" => turn_on,
            "turn off" => turn_off,
            "toggle" => toggle,
            _ => panic!(),
        };
        let val = |i| c.get(i).unwrap().as_str().parse().unwrap();
        f(&mut g, &mut b, (val(2), val(3)), (val(4), val(5)));
    });

    println!("Part 1: {}", g.iter().flatten().sum::<u32>());
    println!("Part 2: {}", b.iter().flatten().sum::<u32>());
}
