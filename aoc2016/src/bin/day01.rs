use std::{collections::HashSet, fs};

#[derive(Default)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Default)]
struct Player {
    pos: (i32, i32),
    facing: Direction,
    blocks: HashSet<(i32, i32)>,
    twice: Option<(i32, i32)>,
}

impl Player {
    fn move_as_instructed(mut self, instruc: &str) -> Self {
        self.facing = match instruc.chars().nth(0) {
            Some('L') => self.facing.turn_left(),
            Some('R') => self.facing.turn_right(),
            _ => panic!("Invalid instruction"),
        };
        let blocks: i32 = instruc[1..].parse().unwrap();
        (0..blocks).for_each(|_| self.move_one_block());
        self
    }

    fn move_one_block(&mut self) {
        match self.facing {
            Direction::North => self.pos.1 += 1,
            Direction::South => self.pos.1 -= 1,
            Direction::East => self.pos.0 += 1,
            Direction::West => self.pos.0 -= 1,
        }
        if !self.blocks.insert(self.pos) && self.twice.is_none() {
            self.twice = Some(self.pos);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let player = input
        .trim()
        .split(", ")
        .fold(Player::default(), Player::move_as_instructed);

    let last_pos = player.pos;
    println!("Part 1: {}", last_pos.0.abs() + last_pos.1.abs());
    let hq_loc = player.twice.unwrap();
    println!("Part 2: {}", hq_loc.0.abs() + hq_loc.1.abs());
}
