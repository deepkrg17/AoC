use std::{fs, num::ParseIntError, str::FromStr};

struct GiftBox {
    length: u32,
    width: u32,
    height: u32,
}

impl GiftBox {
    fn sides(&self) -> [u32; 3] {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    fn perimeters(&self) -> [u32; 3] {
        [
            (self.length + self.width) * 2,
            (self.width + self.height) * 2,
            (self.height + self.length) * 2,
        ]
    }

    fn paper_needed(&self) -> u32 {
        let surface = self.sides().iter().sum::<u32>() * 2;
        let min_side = self.sides().into_iter().min().unwrap();
        surface + min_side
    }

    fn ribbon_needed(&self) -> u32 {
        let volume = self.length * self.width * self.height;
        let min_perimeter = self.perimeters().into_iter().min().unwrap();
        volume + min_perimeter
    }
}

impl FromStr for GiftBox {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split('x');
        Ok(GiftBox {
            length: dimensions.next().unwrap().parse()?,
            width: dimensions.next().unwrap().parse()?,
            height: dimensions.next().unwrap().parse()?,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let gift_boxes = input
        .lines()
        .map(|line| line.parse::<GiftBox>().unwrap())
        .collect::<Vec<GiftBox>>();

    println!(
        "Paper needed: {}",
        gift_boxes.iter().map(GiftBox::paper_needed).sum::<u32>()
    );

    println!(
        "Ribbon needed: {}",
        gift_boxes.iter().map(GiftBox::ribbon_needed).sum::<u32>()
    );
}
