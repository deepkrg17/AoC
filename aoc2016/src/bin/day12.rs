use std::fs;

fn get_index(r: &str) -> usize {
    match r {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => unreachable!(),
    }
}

fn get_value(r: &str, registers: &[i32; 4]) -> i32 {
    match r.parse::<i32>() {
        Ok(v) => v,
        Err(_) => registers[get_index(r)],
    }
}

fn solve(instructions: &Vec<Vec<&str>>) -> i32 {
    let mut regs = [0; 4];
    let mut idx = 0;

    while idx < instructions.len() {
        let mut ins = instructions[idx].iter().cloned();
        let mut next = || ins.next().unwrap();
        match next() {
            "cpy" => {
                let val = get_value(next(), &regs);
                let reg = get_index(next());
                regs[reg] = val;
            }
            "inc" => {
                let reg = get_index(next());
                regs[reg] += 1;
            }
            "dec" => {
                let reg = get_index(next());
                regs[reg] -= 1;
            }
            "jnz" if get_value(next(), &regs) != 0 => {
                let offset: i32 = next().parse().unwrap();
                idx = (idx as i32 + offset) as usize;
                continue;
            }
            _ => (),
        }
        idx += 1;
    }
    regs[0]
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut instructions = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(&instructions));

    instructions.insert(0, vec!["cpy", "1", "c"]);
    println!("Part 2: {}", solve(&instructions));
}
