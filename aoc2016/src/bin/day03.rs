use std::fs;

fn is_valid_triangle(vec: &[u32; 3]) -> bool {
    let mut sides = vec.to_owned();
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sides = input
        .lines()
        .map(|line| {
            /* line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>() */
            let mut split = line.split_whitespace();
            let a = split.next().unwrap().parse::<u32>().unwrap();
            let b = split.next().unwrap().parse::<u32>().unwrap();
            let c = split.next().unwrap().parse::<u32>().unwrap();
            [a, b, c]
        })
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        sides.iter().cloned().filter(is_valid_triangle).count()
    );

    println!(
        "Part 2: {}",
        sides
            .chunks(3)
            .flat_map(|x| {
                [
                    [x[0][0], x[1][0], x[2][0]],
                    [x[0][1], x[1][1], x[2][1]],
                    [x[0][2], x[1][2], x[2][2]],
                ]
            })
            .filter(is_valid_triangle)
            .count()
    );
}
