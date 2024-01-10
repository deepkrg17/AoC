use std::{collections::HashSet, fs};

#[derive(Default)]
struct Santa {
    x: i32,
    y: i32,
}

impl Santa {
    fn move_in_direction(&mut self, c: char) {
        match c {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => (),
        }
    }

    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut houses = HashSet::<(i32, i32)>::new();
    let mut santa = Santa::default();
    houses.insert(santa.pos());

    let mut houses_with_robo = HashSet::new();
    let mut santa_with_robo = Santa::default();
    let mut robo = Santa::default();
    houses_with_robo.insert(santa_with_robo.pos());

    input.chars().enumerate().for_each(|(i, c)| {
        santa.move_in_direction(c);
        houses.insert(santa.pos());
        if i % 2 == 0 {
            robo.move_in_direction(c);
            houses_with_robo.insert(robo.pos());
        } else {
            santa_with_robo.move_in_direction(c);
            houses_with_robo.insert(santa_with_robo.pos());
        };
    });

    println!("Part 1: {}", houses.len());
    println!("Part 2: {}", houses_with_robo.len());
}
