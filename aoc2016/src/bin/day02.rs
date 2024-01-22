fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut code = String::new();
    let mut x = 1_usize;
    let mut y = 1_usize;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => y = (y + 1).min(2),
                'L' => x = x.saturating_sub(1),
                'R' => x = (x + 1).min(2),
                _ => panic!("Invalid input"),
            }
        }
        code.push(keypad[y][x]);
    }
    println!("Part 1: {}", code);

    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    code.clear();
    (x, y) = (0, 2);
    for line in input.lines() {
        for c in line.chars() {
            let (x_, y_) = match c {
                'U' => (x, y.saturating_sub(1)),
                'D' => (x, (y + 1).min(4)),
                'L' => (x.saturating_sub(1), y),
                'R' => ((x + 1).min(4), y),
                _ => panic!("Invalid input"),
            };
            if keypad[y_][x_] != ' ' {
                x = x_;
                y = y_;
            }
        }
        code.push(keypad[y][x]);
    }
    println!("Part 2: {}", code);
}
