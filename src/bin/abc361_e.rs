use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n-1],
    }

    let mut path: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut ans: usize = 0;
    let mut branch = false;
    for v in abc.iter() {
        let (a, b, c) = (v[0], v[1], v[2]);
        let val = path.entry(a).or_insert(vec![]);
        val.push((b, c));
        let val = path.entry(b).or_insert(vec![]);
        val.push((a, c));
        ans += c;
    }
    
    // 木の直径を算出する(BFS2回)
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    let mut dist: Vec<usize> = vec![0; n+1];
    que.push_front(1);
    visited[1] = true;
    while !que.is_empty() {
        let i = que.pop_back().unwrap();
        let d = dist[i];
        for (b, c) in path[&i].iter() {
            if !visited[*b] {
                visited[*b] = true;
                que.push_front(*b);
                dist[*b] = d + c;
            }
        }
    }
    let mut max_i = 0;
    let mut max_d = 0;
    for (i, d) in dist.iter().enumerate() {
        if max_d < *d {
            max_i = i;
            max_d = *d;
        }
    }

    // 2回目
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    let mut dist: Vec<usize> = vec![0; n+1];
    que.push_front(max_i);
    visited[max_i] = true;
    while !que.is_empty() {
        let i = que.pop_back().unwrap();
        let d = dist[i];
        for (b, c) in path[&i].iter() {
            if !visited[*b] {
                visited[*b] = true;
                que.push_front(*b);
                dist[*b] = d + c;
            }
        }
    }
    let mut max_i2 = 0;
    let mut max_d2 = 0;
    for (i, d) in dist.iter().enumerate() {
        if max_d2 < *d {
            max_i2 = i;
            max_d2 = *d;
        }
    }

    let mut ans = 0;
    for v in abc.iter() {
        ans += v[2];
    }

    println!("{}", ans*2 - max_d2);
}