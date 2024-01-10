use std::{fs, usize};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (floor, base) = input
        .chars()
        .enumerate()
        .fold((0, usize::MAX), |(floor, base), (i, c)| match c {
            '(' => (floor + 1, base),
            ')' if floor == 0 => (floor - 1, base.min(i + 1)),
            ')' => (floor - 1, base),
            _ => (floor, base),
        });

    println!("Part 1: {}", floor);
    println!("Part 2: {}", base);
}
