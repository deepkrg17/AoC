use std::{fs, ops::RangeInclusive};

fn get_range(range: &str) -> RangeInclusive<usize> {
    let (first, last) = range.trim().split_once("-").unwrap();
    let first = first.parse().unwrap();
    let last = last.parse().unwrap();
    first..=last
}

fn is_invalid_n(id: usize) -> bool {
    let id = id.to_string();
    let mid = id.len() / 2;
    let (p1, p2) = id.split_at(mid);
    p1 == p2
}

fn is_invalid_s(id: usize) -> bool {
    let num = id.to_string();
    let length = num.len();

    for i in 1..=length / 2 {
        if length.is_multiple_of(i) {
            let sequence = &num[..i];
            let n = length / i;
            if sequence.repeat(n) == num {
                return true;
            }
        }
    }
    false
}

fn add_invalid_ids(acc: (usize, usize), range: &str) -> (usize, usize) {
    get_range(range).fold(acc, |(acc1, acc2), id| {
        (
            if is_invalid_n(id) { acc1 + id } else { acc1 },
            if is_invalid_s(id) { acc2 + id } else { acc2 },
        )
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let added = input.split(",").fold((0, 0), add_invalid_ids);
    println!("{added:?}");
}
