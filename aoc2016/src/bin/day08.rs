fn fill_rect(screen: &mut [[bool; 50]; 6], xy: &str) {
    let mut split = xy.split('x');
    let x: usize = split.next().unwrap().parse().unwrap();
    let y: usize = split.next().unwrap().parse().unwrap();
    for row in screen.iter_mut().take(y) {
        for px in row.iter_mut().take(x) {
            *px = true;
        }
    }
}

fn show_screen(screen: &[[bool; 50]; 6]) {
    for row in screen.iter() {
        for px in row.iter() {
            print!("{}", if *px { '#' } else { ' ' });
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut screen = [[false; 50]; 6];

    input.lines().for_each(|line| {
        if line.starts_with("rect") {
            return fill_rect(&mut screen, &line[5..]);
        }
        let mut split = line[7..].split(" by ");
        let mut dir = split.next().unwrap().split('=');
        let part = &dir.next().unwrap()[..3];
        let idx: usize = dir.next().unwrap().parse().unwrap();
        let n: usize = split.next().unwrap().parse().unwrap();
        match part {
            "row" => screen[idx].rotate_right(n),
            "col" => {
                let mut col = [false; 6];
                for (i, row) in screen.iter().enumerate() {
                    col[(i + n) % 6] = row[idx];
                }
                for (i, row) in screen.iter_mut().enumerate() {
                    row[idx] = col[i];
                }
            }
            _ => panic!("Unknown direction"),
        }
    });

    let lit = screen.iter().flatten().filter(|&&px| px).count();
    println!("Lit: {}", lit);
    println!("Screen:");
    show_screen(&screen);
}
