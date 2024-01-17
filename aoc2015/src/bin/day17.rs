fn count_combinations(sizes: &[u32], idx: usize, used: u32, num: usize, count: &mut Vec<u32>) {
    if used == 150 {
        count[num] += 1;
    } else if idx < sizes.len() {
        count_combinations(sizes, idx + 1, used, num, count);
        if used + sizes[idx] <= 150 {
            count_combinations(sizes, idx + 1, used + sizes[idx], num + 1, count);
        }
    }
}

fn main() {
    let sizes: Vec<u32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    // Count of combinations grouped by the num of containers used.
    let mut count_by_num = vec![0; sizes.len() + 1];

    count_combinations(&sizes, 0, 0, 0, &mut count_by_num);

    println!("Part 1: {}", count_by_num.iter().sum::<u32>());
    println!("Part 2: {}", count_by_num.iter().find(|&&c| c > 0).unwrap());
}
