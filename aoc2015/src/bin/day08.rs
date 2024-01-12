use std::fs;

struct Info {
    diff: usize,
    encode: usize,
}

impl Info {
    fn parse(s: &str) -> Info {
        let mut s_c = s.chars();
        let mut in_mem = 0usize;
        let mut encode = 4usize;

        while let Some(c) = s_c.next() {
            match c {
                '"' => (),
                '\\' => {
                    match s_c.next().unwrap() {
                        '"' | '\\' => encode += 1,
                        'x' => {
                            s_c.next();
                            s_c.next();
                        }
                        x => panic!("Invalid escape : {}", x),
                    }
                    in_mem += 1;
                    encode += 1;
                }
                _ => in_mem += 1,
            }
        }

        Info {
            diff: s.len() - in_mem,
            encode,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (a, b) = input
        .lines()
        .map(Info::parse)
        .fold((0, 0), |(a, b), x| (a + x.diff, b + x.encode));
    println!("Part 1: {}", a);
    println!("Part 2: {}", b);
}
