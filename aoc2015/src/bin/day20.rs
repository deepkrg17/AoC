fn sum_factors(n: usize, only_50: bool) -> usize {
    (1..=(n as f64).sqrt() as usize)
        .filter(|&i| n % i == 0)
        .map(|i| {
            if !only_50 {
                i + n / i
            } else {
                (if i > 50 { 0 } else { n / i } + if n / i > 50 { 0 } else { i })
            }
        })
        .sum()
}

fn main() {
    let input = 33100000;

    println!(
        "Part 1: {}",
        (1..)
            .find(|&i| sum_factors(i, false) * 10 >= input)
            .unwrap()
    );

    println!(
        "Part 2: {}",
        (1..).find(|&i| sum_factors(i, true) * 11 >= input).unwrap()
    );
}
