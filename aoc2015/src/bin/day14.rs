use std::{cmp::Ordering, fs};

#[derive(Eq, PartialEq)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    cycle_time: u32,
    distance: u32,
    points: u32,
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Reindeer {
    fn parse(input: &str) -> Reindeer {
        let mut words = input.split_whitespace();
        let speed: u32 = words.nth(3).unwrap().parse().unwrap();
        let fly_time: u32 = words.nth(2).unwrap().parse().unwrap();
        let resttime: u32 = words.nth(6).unwrap().parse().unwrap();
        Reindeer {
            speed,
            fly_time,
            cycle_time: fly_time + resttime,
            distance: 0,
            points: 0,
        }
    }

    fn resting(&self, time: u32) -> bool {
        time % self.cycle_time >= self.fly_time
    }

    fn tick(&mut self, time: u32) {
        if !self.resting(time) {
            self.distance += self.speed
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut deers = input.lines().map(Reindeer::parse).collect::<Vec<_>>();

    (0..2503_u32).for_each(|t| {
        deers.iter_mut().for_each(|deer| deer.tick(t));
        let max_dist = deers.iter().max().unwrap().distance;
        deers.iter_mut().for_each(|deer| {
            if deer.distance == max_dist {
                deer.points += 1;
            }
        });
    });

    println!("Part 1: {}", deers.iter().max().unwrap().distance);
    println!(
        "Part 2: {}",
        deers.iter().fold(0, |acc, r| acc.max(r.points))
    );
}
