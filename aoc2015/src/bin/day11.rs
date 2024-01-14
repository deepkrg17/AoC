use std::collections::HashSet;

fn next_password(pass: String) -> String {
    let mut next = inc_password(pass);
    while !is_valid_password(&next) {
        next = inc_password(next);
    }
    next
}

fn is_valid_password(p: &str) -> bool {
    !['i', 'o', 'l'].iter().any(|&c| p.contains(c))
        && p.as_bytes()
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
        && p.as_bytes()
            .windows(2)
            .filter(|w| w[0] == w[1])
            .collect::<HashSet<_>>()
            .len()
            >= 2
}

fn inc_password(p: String) -> String {
    let mut bytes = p.into_bytes();
    for i in bytes.iter_mut().rev() {
        if *i == b'z' {
            *i = b'a';
        } else {
            *i += 1;
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn main() {
    let input = "vzbxkghb".to_string();

    let pass1 = next_password(input);
    println!("Part 1: {}", pass1);
    println!("Part 2: {}", next_password(pass1));
}
