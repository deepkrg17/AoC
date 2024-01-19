use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut splits = input.split("\n\n");

    let replacements = splits.next().unwrap();
    let molecule = splits.next().unwrap().trim();
    let mut molecules = HashSet::new();

    replacements.lines().for_each(|line| {
        let mut split = line.split(" => ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        for (i, _) in molecule.match_indices(from) {
            molecules.insert(format!(
                "{}{}",
                &molecule[..i],
                molecule[i..].replacen(from, to, 1)
            ));
        }
    });
    println!("Part 1: {}", molecules.len());

    // https://www.reddit.com/r/adventofcode/s/mqvQ7jsuQR
    let c = molecule.chars().filter(|c| c.is_uppercase()).count()
        - molecule.matches("Rn").count()
        - molecule.matches("Ar").count()
        - 2 * molecule.matches('Y').count()
        - 1;
    println!("Part 2: {}", c);
}
