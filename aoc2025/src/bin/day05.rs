use std::ops::RangeInclusive;

fn get_range(range: &str) -> RangeInclusive<usize> {
    let (first, last) = range.split_once("-").unwrap();
    let first = first.parse().unwrap();
    let last = last.parse().unwrap();
    first..=last
}

fn main() {
    let input = include_str!("./input.txt");
    let (rs, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = rs.lines().map(get_range).collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r| std::cmp::Reverse(*r.start()));

    println!(
        "{}",
        ids.lines()
            .map(|x| x.parse().unwrap())
            .filter(|id| ranges.iter().any(|r| r.contains(id)))
            .count()
    );

    let mut merged = Vec::with_capacity(ranges.len() / 2);
    let mut cur = ranges.pop().unwrap();
    while let Some(r) = ranges.pop() {
        if cur.contains(r.start()) {
            cur = *cur.start()..=*cur.end().max(r.end());
        } else {
            merged.push(cur);
            cur = r;
        }
    }
    merged.push(cur);
    println!(
        "{}",
        merged
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<usize>()
    );
}
