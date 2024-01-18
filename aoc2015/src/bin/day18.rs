type Grid = [[u16; 100]; 100];
const CORNERS: [(usize, usize); 4] = [(0, 0), (0, 99), (99, 0), (99, 99)];

fn get_adjacent_count(grid: &Grid, x: usize, y: usize) -> u16 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let x = x as i32 + dx;
            let y = y as i32 + dy;
            if (0..100).contains(&x) && (0..100).contains(&y) {
                count += grid[y as usize][x as usize];
            }
        }
    }
    count - grid[y][x]
}

fn next_step(grid: &Grid, stuck: bool) -> Grid {
    let mut new_grid = *grid;
    for y in 0..100 {
        for x in 0..100 {
            if stuck && CORNERS.contains(&(x, y)) {
                continue;
            }
            let c = get_adjacent_count(grid, x, y);
            new_grid[y][x] = match (grid[y][x], c) {
                (1, 2 | 3) => 1,
                (0, 3) => 1,
                _ => 0,
            }
        }
    }
    new_grid
}

fn main() {
    let mut grid: Grid = [[0; 100]; 100];
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                grid[y][x] = if c == '#' { 1 } else { 0 };
            })
        });

    println!(
        "Part 1: {}",
        (0..100)
            .fold(grid, |grid, _| next_step(&grid, false))
            .iter()
            .map(|row| row.iter().sum::<u16>())
            .sum::<u16>()
    );

    CORNERS.iter().for_each(|&(x, y)| {
        grid[y][x] = 1;
    });
    println!(
        "Part 2: {}",
        (0..100)
            .fold(grid, |grid, _| next_step(&grid, true))
            .iter()
            .map(|row| row.iter().sum::<u16>())
            .sum::<u16>()
    );
}
