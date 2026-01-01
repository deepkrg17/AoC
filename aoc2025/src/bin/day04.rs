type Grid = Vec<Vec<char>>;
type CoOrd = (usize, usize);

fn get_adjacent_coords(x: usize, y: usize) -> Vec<CoOrd> {
    let mut adjacent = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                adjacent.push((x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)));
            }
        }
    }

    adjacent
}

fn calc_acbl_rolls(grid: &Grid) -> u32 {
    let mut accessable_rolls = 0;
    for y in 0..grid.len() {
        for x in 0..grid[1].len() {
            if grid[y][x] == '.' {
                continue;
            }
            if get_adjacent_coords(x, y)
                .into_iter()
                .filter(|&(ax, ay)| {
                    grid.get(ay)
                        .and_then(|r| r.get(ax))
                        .is_some_and(|&c| c == '@')
                })
                .count()
                < 4
            {
                accessable_rolls += 1;
            }
        }
    }
    accessable_rolls
}

fn calc_acbl_rolls_r(grid: &mut Grid) -> u32 {
    let mut acbl_rolls = 0;
    let mut queue: Vec<CoOrd> = (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .collect::<Vec<CoOrd>>();
    while let Some((x, y)) = queue.pop() {
        if grid[y][x] == '.' {
            continue;
        }
        if get_adjacent_coords(x, y)
            .into_iter()
            .filter(|&(ax, ay)| {
                grid.get(ay)
                    .and_then(|r| r.get(ax))
                    .is_some_and(|&c| c == '@')
            })
            .count()
            < 4
        {
            acbl_rolls += 1;
            grid[y][x] = '.';
            queue.extend(get_adjacent_coords(x, y).into_iter().filter(|&(ax, ay)| {
                grid.get(ay)
                    .and_then(|r| r.get(ax))
                    .is_some_and(|&c| c == '@')
            }))
        }
    }
    acbl_rolls
}

fn main() {
    let binding = include_str!("input.txt");
    let grid: Grid = binding.lines().map(|line| line.chars().collect()).collect();
    let mut gridc = grid.clone();

    let output: u32 = calc_acbl_rolls(&grid);
    println!("{output}");

    let output2: u32 = calc_acbl_rolls_r(&mut gridc);
    println!("{output2}");
}
