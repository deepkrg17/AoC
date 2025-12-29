use std::fs;

fn rotate(point: i32, rotation: &str) -> (i32, i32) {
    let (dir, val) = rotation.split_at(1);
    let clicks = val.parse::<i32>().unwrap();
    let (mut zeros, extras) = (clicks / 100, clicks % 100);

    let n_point = if dir == "R" {
        point + extras
    } else {
        point - extras
    };

    if n_point >= 100 {
        (zeros + 1, n_point - 100)
    } else if n_point < 0 {
        // if it wasn't pointed at 0, it goes through 0
        zeros += (point != 0) as i32;
        (zeros, n_point + 100)
    } else {
        zeros += (n_point == 0) as i32;
        (zeros, n_point)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut pass1 = 0;
    let mut pass2 = 0;
    input.lines().fold(50, |acc, x| {
        let (zeros, point) = rotate(acc, x);
        pass1 += (point == 0) as usize;
        pass2 += zeros;
        point
    });
    println!("{pass1}\n{pass2}");
}
