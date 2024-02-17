use std::fs;

fn count_safe_tiles(mut prev_row: String, rows: usize) -> usize {
    let mut count = prev_row.matches('.').count();
    for _ in 0..(rows - 1) {
        prev_row = format!(".{}.", prev_row)
            .as_bytes()
            .windows(3)
            .map(|w| if w[0] != w[2] { '^' } else { '.' })
            .collect();
        count += prev_row.matches('.').count();
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let prev_row = input.trim().to_string();
    println!("Part 1: {}", count_safe_tiles(prev_row.clone(), 40));
    println!("Part 2: {}", count_safe_tiles(prev_row, 400_000));
}
