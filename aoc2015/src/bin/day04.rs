fn find_hash(key: &str, mask: u8) -> usize {
    (1..)
        .map(|i| md5::compute(format!("{}{}", key, i)))
        .position(|d| d[0] == 0 && d[1] == 0 && d[2] & mask == 0)
        .unwrap()
        + 1
}

fn main() {
    let key = "yzbqklnj";
    println!("part 1: {:?}", find_hash(key, 0xF0));
    println!("part 2: {:?}", find_hash(key, 0xFF));
}
