use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(usize, usize, i64, i64); m],
    }
    let mut link: HashMap<usize, Vec<(usize, i64, i64)>> = HashMap::new();
    for a in 0..n {
        link.insert(a, vec![]);
    }
    for (a, b, x, y) in abxy.iter() {
        link.get_mut(&(a-1)).unwrap().push((b-1, *x, *y));
        link.get_mut(&(b-1)).unwrap().push((a-1, -x, -y));
    }
    let mut ans: Vec<(i64, i64)> = vec![(i64::MAX, i64::MAX); n];
    let mut que: VecDeque<(usize, i64, i64)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n];
    que.push_front((0, 0, 0));
    visited[0] = true;
    ans[0] = (0, 0);
    while !que.is_empty() {
        let (a, x, y) = que.pop_back().unwrap();
        for (b, dx, dy) in link.get(&a).unwrap().iter() {
            if !visited[*b] {
                let (x2, y2) = (x+dx, y+dy);
                que.push_front((*b, x2, y2));
                ans[*b] = (x2, y2);
                visited[*b] = true;
            }
        }
    }
    for (x, y) in ans.iter() {
        if *x == i64::MAX {
            println!("undecidable");
        } else {
            println!("{} {}", x, y);
        }
    }
}