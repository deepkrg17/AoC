fn is_open(x: usize, y: usize) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + 1350).count_ones() % 2 == 0
}

fn main() {
    // let v = vec![vec![0; 50]; 50];

    for y in 0..50 {
        for x in 0..50 {
            match (x, y) {
                (1, 1) => print!("󰮯"),
                (31, 39) => print!("󰏖"),
                _ if is_open(x, y) => print!("·"),
                _ => print!("-"),
            };
        }
        println!();
    }
}
