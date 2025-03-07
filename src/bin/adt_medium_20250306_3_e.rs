use proconio::input;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a_set: HashSet<_> = a.clone().into_iter().collect();
    let mut a2: Vec<_> = a_set.into_iter().collect();
    a2.sort();
    let k_max = a2.len()-1;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (i, &ai) in a2.iter().enumerate() {
        map.insert(k_max-i, ai);
    }
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        let e = cnt.entry(ai).or_insert(0);
        *e += 1;
    }
    for k in 0..n {
        if let Some(ai) = map.get(&k) {
            let ans = cnt.get(ai).unwrap();
            println!("{}", ans);
        } else {
            println!("0");
        }
    }
}