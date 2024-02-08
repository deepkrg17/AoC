fn calc_time(discs: &[(u32, u32)]) -> u32 {
    let top = discs[0];
    let mut time = top.0 - top.1;

    while discs
        .iter()
        .enumerate()
        .any(|(i, disc)| (time + i as u32 + disc.1) % disc.0 != 0)
    {
        time += top.0;
    }
    time - 1
}

fn main() {
    let mut discs = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            (
                splits.nth(3).unwrap().parse::<u32>().unwrap(),
                splits
                    .nth(7)
                    .unwrap()
                    .trim_matches('.')
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();

    println!("Part 1: {}", calc_time(&discs));
    discs.push((11, 0));
    println!("Part 2: {}", calc_time(&discs));
}
