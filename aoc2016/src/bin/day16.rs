fn modify_data(mut data: String, length: usize) -> String {
    while data.len() < length {
        let mut modified_data = data.to_string() + "0";
        for c in data.chars().rev() {
            match c {
                '0' => modified_data.push('1'),
                _ => modified_data.push('0'),
            }
        }
        data = modified_data;
    }
    data.truncate(length);
    data
}

fn calc_checksum(data: &str) -> String {
    let checksum = data
        .as_bytes()
        .chunks(2)
        .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
        .collect::<String>();
    if checksum.len() % 2 == 0 {
        calc_checksum(&checksum)
    } else {
        checksum
    }
}

fn main() {
    let data = "10001110011110000";
    let modified_data = modify_data(data.to_string(), 272);
    println!("Part 1: {}", calc_checksum(&modified_data));
    let modified_data = modify_data(data.to_string(), 35651584);
    println!("Part 2: {}", calc_checksum(&modified_data));
}
