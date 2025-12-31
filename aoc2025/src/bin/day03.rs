use std::cmp::max;

fn calc_largest_jolt(bank: &str) -> u32 {
    let mut tens = 0;
    let mut largest_jolt = 0;

    for c in bank.chars() {
        let cur = c.to_digit(10).unwrap();
        largest_jolt = max(largest_jolt, tens * 10 + cur);
        tens = max(tens, cur);
    }

    largest_jolt
}

fn calc_largest_jolt_n(bank: &str) -> usize {
    let mut b_bank = bank.as_bytes();
    let f = |b_bank: &[u8]| {
        b_bank.iter().enumerate().fold(
            (0, b'0'),
            |acc, (i, &b)| if b > acc.1 { (i, b) } else { acc },
        )
    };

    (0..12)
        .rev()
        .map(|p| {
            let (i, n) = f(&b_bank[..b_bank.len() - p]);
            b_bank = &b_bank[i + 1..];
            (n - b'0') as usize * 10usize.pow(p as u32)
        })
        .sum()
}

fn main() {
    let binding = include_str!("input.txt");
    let banks = binding.lines();
    let banks_n = binding.lines();

    let output: u32 = banks.map(calc_largest_jolt).sum();
    let output_n: usize = banks_n.map(calc_largest_jolt_n).sum();

    println!("{output}, {output_n}");
}
