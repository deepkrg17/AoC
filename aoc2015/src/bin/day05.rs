use std::{fs, str::from_utf8};

fn is_nice_old(&s: &&str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let disallowed = ["ab", "cd", "pq", "xy"];
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
        && disallowed.iter().all(|w| !s.contains(w))
        && s.chars().filter(|c| vowels.contains(c)).count() >= 3
}

fn is_nice_new(&s: &&str) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
        && s.as_bytes()
            .windows(2)
            .any(|w| s.matches(from_utf8(w).unwrap()).count() >= 2)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", input.lines().filter(is_nice_old).count());
    println!("Part 2: {}", input.lines().filter(is_nice_new).count());
}
