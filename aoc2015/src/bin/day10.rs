fn look_and_say(input: &str) -> String {
    let mut cur = input.chars().next().unwrap();
    let mut count = 0;
    let mut acc = String::new();
    for c in input.chars() {
        if cur == c {
            count += 1;
        } else {
            acc += &format!("{}{}", count, cur);
            cur = c;
            count = 1;
        }
    }
    acc + &format!("{}{}", count, cur)
}

fn main() {
    let inp = "3113322113".to_string();
    let out = (0..40).fold(inp, |acc, _| look_and_say(&acc));
    println!("Part 1: {}", out.len());
    let out = (0..10).fold(out, |acc, _| look_and_say(&acc));
    println!("Part 2: {}", out.len());
}
