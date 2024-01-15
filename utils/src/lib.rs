pub fn get_permutations<T: Clone>(v: Vec<T>) -> Vec<Vec<T>> {
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
