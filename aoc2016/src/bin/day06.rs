fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut freq = vec![[0; 26]];

    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if freq.len() <= i {
                freq.push([0; 26]);
            }
            freq[i][(c as u8 - b'a') as usize] += 1;
        });
    });

    println!(
        "Part 1: {}",
        freq.iter()
            .map(
                |f| (f.iter().enumerate().max_by_key(|(_, &val)| val).unwrap().0 as u8 + b'a')
                    as char
            )
            .collect::<String>()
    );

    println!(
        "Part 2: {}",
        freq.iter()
            .map(
                |f| (f.iter().enumerate().min_by_key(|(_, &val)| val).unwrap().0 as u8 + b'a')
                    as char
            )
            .collect::<String>()
    );
}
