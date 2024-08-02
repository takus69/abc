use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 1..=(n1+n2) {
        link.insert(i, vec![]);
    }
    for (ai, bi) in ab.iter() {
        let e = link.entry(*ai).or_insert(vec![]);
        e.push(*bi);
        let e = link.entry(*bi).or_insert(vec![]);
        e.push(*ai);
    }
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n1];
    que.push_front((1, 0));
    visited[0] = true;
    let mut ans1 = 0;
    while !que.is_empty() {
        let (v, d) = que.pop_back().unwrap();
        for next in link[&v].iter() {
            if visited[next-1] { continue; }
            visited[next-1] = true;
            que.push_front((*next, d+1));
            ans1 = ans1.max(d+1);
        }
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n2];
    que.push_front((n1+n2, 0));
    visited[n2-1] = true;
    let mut ans2 = 0;
    while !que.is_empty() {
        let (v, d) = que.pop_back().unwrap();
        for next in link[&v].iter() {
            if visited[next-n1-1] { continue; }
            visited[next-n1-1] = true;
            que.push_front((*next, d+1));
            ans2 = ans2.max(d+1);
        }
    }
    println!("{}", ans1+ans2+1);

}