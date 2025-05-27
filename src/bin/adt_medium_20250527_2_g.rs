use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(usize, usize, isize, isize); m],
    }
    let mut map: HashMap<(usize, usize), (isize, isize)> = HashMap::new();
    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b, x, y) in abxy.iter() {
        map.insert((a, b), (x, y));
        map.insert((b, a), (-x, -y));
        let e = link.entry(a).or_insert(Vec::new());
        e.push(b);
        let e = link.entry(b).or_insert(Vec::new());
        e.push(a);
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut ans: Vec<(isize, isize)> = vec![(isize::MAX, isize::MAX); n+1];
    let mut visited: Vec<bool> = vec![false; n+1];
    ans[1] = (0, 0);
    que.push_front(1);
    visited[1] = true;
    let mut cnt = 1;
    while !que.is_empty() {
        let a = que.pop_back().unwrap();
        let (x, y) = ans[a];
        for &b in link.get(&a).unwrap_or(&Vec::new()) {
            if visited[b] { continue; }
            if !map.contains_key(&(a, b)) { continue; }
            let &(dx, dy) = map.get(&(a, b)).unwrap();
            let (x2, y2) = (x+dx, y+dy);
            ans[b] = (x2, y2);
            visited[b] = true;
            que.push_front(b);
            cnt += 1;
        }
    }

    for i in 1..=n {
        if visited[i] {
            println!("{} {}", ans[i].0, ans[i].1);
        } else {
            println!("undecidable");
        }
    }
}