use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize, usize, usize); m],
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
    for &(a, b, w) in &abw {
        graph[a].push((b, w));
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 1<<10]; n+1];
    que.push_back((1, 0));
    visited[1][0] = true;
    while let Some((a, s)) = que.pop_front() {
        for &(b, w) in graph[a].iter() {
            let next_s = s ^ w;
            if visited[b][next_s] { continue; }
            que.push_back((b, next_s));
            visited[b][next_s] = true;
        }
    }

    for j in 0..(1<<10) {
        if visited[n][j] {
            println!("{}", j);
            return;
        }
    }
    println!("-1");
}