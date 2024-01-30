fn decompress(input: &str) -> (usize, usize) {
    let mut len1 = 0;
    let mut len2 = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let marker = (&mut chars).take_while(|c| *c != ')').collect::<String>();
            let (n, r) = marker.split_once('x').unwrap();
            let n = n.parse::<usize>().unwrap();
            let r = r.parse::<usize>().unwrap();
            let seq = (&mut chars).take(n).collect::<String>();

            len1 += r * n;
            len2 += r * decompress(&seq).1;
        } else {
            len1 += 1;
            len2 += 1;
        }
    }
    (len1, len2)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (len1, len2) = decompress(input.trim());
    println!("Decompressed length (p1): {}", len1);
    println!("Decompressed length (p2): {}", len2);
}
