use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n-1],
        v: [usize; k],
    }
    if n == 1 {
        println!("1");
        std::process::exit(0);
    }
    // 葉からvを含まないものを全て削除する
    let mut set: HashSet<usize> = HashSet::new();
    for vi in v {
        set.insert(vi-1);
    }
    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (ai, bi) in ab.iter() {
        let e = map.entry(ai-1).or_insert(HashSet::new());
        e.insert(bi-1);
        let e = map.entry(bi-1).or_insert(HashSet::new());
        e.insert(ai-1);
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        let l = map.get(&i).unwrap();
        if l.len() == 1 {
            que.push_back(i);
        }
    }

    let mut ans = n;
    let mut del: Vec<bool> = vec![false; n];
    while !que.is_empty() {
        let u = que.pop_back().unwrap();
        // println!("que u: {}", u);
        if del[u] { continue; }
        if !set.contains(&u) {
            ans -= 1;
            del[u] = true;
            // println!("del u: {}", u);
            let l = map.get(&u).unwrap().clone();
            for v in l.iter() {
                if del[*v] { continue; }
                let e = map.get_mut(&v).unwrap();
                e.remove(&u);
                if e.len() == 1 { que.push_front(*v); }
            }
        }
    }

    println!("{}", ans);
}