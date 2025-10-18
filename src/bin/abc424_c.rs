use proconio::input;
use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut link: Vec<HashSet<usize>> = vec![HashSet::new(); n+1];
    let mut skill: HashSet<usize> = HashSet::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        if (a, b) == (0, 0) {
            skill.insert(i+1);
        } else {
            link[a].insert(i+1);
            link[b].insert(i+1);
        }
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for &s in skill.iter() {
        que.push_front(s);
    }
    while let Some(a) = que.pop_back() {
        for &b in link[a].iter() {
            if skill.contains(&b) { continue; }
            skill.insert(b);
            que.push_front(b);
        }
    }
    
    println!("{}", skill.len());
}