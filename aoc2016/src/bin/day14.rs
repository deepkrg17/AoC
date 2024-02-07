fn get_hash(idx: usize, hashes: &mut Vec<String>, stretch: usize) -> String {
    if idx < hashes.len() {
        hashes[idx].clone()
    } else {
        let hash = (0..=stretch).fold(format!("jlmsuwbz{idx}"), |hash, _| {
            format!("{:x}", md5::compute(hash))
        });
        hashes.push(hash.clone());
        hash
    }
}

fn get_tripple(hash: String) -> Option<char> {
    hash.as_bytes().windows(3).find_map(|w| {
        if w[0] == w[1] && w[1] == w[2] {
            Some(w[0] as char)
        } else {
            None
        }
    })
}

fn get_last_key_idx(stretch: usize) -> usize {
    let mut hashes = Vec::new();
    (0..)
        .filter_map(|i| {
            let c = get_tripple(get_hash(i, &mut hashes, stretch))?;
            let c5 = c.to_string().repeat(5);
            print!("{}: {}\r", i, c5);
            (i + 1..i + 1001)
                .find(|&j| get_hash(j, &mut hashes, stretch).contains(&c5))
                .map(|_| i)
        })
        .nth(63)
        .unwrap()
}

fn main() {
    println!("Part 1: {}", get_last_key_idx(0));
    println!("Part 2: {}", get_last_key_idx(2016));
}
