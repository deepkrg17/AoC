use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Paths = HashMap<Path, u32>;

#[derive(Hash, Eq, PartialEq)]
struct Path {
    from: String,
    to: String,
}

fn parse_to_paths(s: &str, paths: &mut Paths, cities: &mut HashSet<String>) {
    let mut split = s.split(" = ");
    let mut path = split.next().unwrap().split(" to ");

    let from = path.next().unwrap().to_string();
    let to = path.next().unwrap().to_string();
    let distance = split.next().unwrap().parse::<u32>().unwrap();

    cities.insert(from.clone());
    cities.insert(to.clone());
    paths.insert(
        Path {
            from: to.clone(),
            to: from.clone(),
        },
        distance,
    );
    paths.insert(Path { from, to }, distance);
}

fn get_permutations<T: Clone>(v: Vec<T>) -> Vec<Vec<T>> {
    match v.len() {
        0 | 1 => vec![v],
        2 => {
            let rev0 = v.last().unwrap().clone();
            let rev1 = v.first().unwrap().clone();
            vec![v, vec![rev0, rev1]]
        }
        _ => {
            let mut permutations = vec![];
            for i in 0..v.len() {
                let mut v2 = v.to_vec();
                v2.swap(0, i);
                let curr = v2.first().unwrap().clone();
                v2.remove(0);
                for mut p in get_permutations(v2.to_vec()) {
                    p.insert(0, curr.clone());
                    permutations.push(p);
                }
            }
            permutations
        }
    }
}

fn routes(paths: Paths, cities: Vec<String>) -> (u32, u32) {
    get_permutations(cities)
        .into_iter()
        .fold((u32::MAX, 0), |(min_dist, max_dist), p| {
            let mut dist = 0;
            for i in 1..p.len() {
                let from = p[i - 1].clone();
                let to = p[i].clone();
                dist += paths.get(&Path { from, to }).unwrap();
            }
            (min_dist.min(dist), max_dist.max(dist))
        })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cities = HashSet::new();

    let paths = input.lines().fold(Paths::new(), |mut paths, line| {
        parse_to_paths(line, &mut paths, &mut cities);
        paths
    });
    let cities = cities.into_iter().collect::<Vec<_>>();

    let (min_dist, max_dist) = routes(paths, cities);
    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", max_dist);
}
