use proconio::input;
use std::collections::{VecDeque, HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b) in ab.iter() {
        let e = link.entry(a).or_insert(Vec::new());
        e.push(b);
        let e = link.entry(b).or_insert(Vec::new());
        e.push(a);
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut ans = 1;
    que.push_front(1);
    visited.insert(1);

    while let Some(a) = que.pop_back() {
        if !link.contains_key(&a) { continue; }
        for &b in link.get(&a).unwrap().iter() {
            if visited.contains(&b) { continue; }
            que.push_front(b);
            visited.insert(b);
            ans = ans.max(b);
        }
    }

    println!("{}", ans);
}