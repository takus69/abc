use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in &ab {
        link[a].push(b);
        link[b].push(a);
    }
    let mut dist: Vec<usize> = vec![usize::MAX; n+1];
    let mut visited: Vec<bool> = vec![false; n+1];
    let mut que: VecDeque<usize> = VecDeque::new();
    dist[1] = 0;
    visited[1] = true;
    que.push_front(1);

    while let Some(i) = que.pop_back() {
        let d = dist[i];
        for &j in &link[i] {
            if visited[j] { continue; }
            visited[j] = true;
            dist[j] = d+1;
            que.push_front(j);
        }
    }
    for i in 1..=n {
        if dist[i] == usize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}