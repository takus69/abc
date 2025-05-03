use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    if m != n {
        println!("No");
        return;
    }
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in ab.iter() {
        edges[a].push(b);
        edges[b].push(a);
    }
    for a in 1..=n {
        if edges[a].len() != 2 {
            println!("No");
            return;
        }
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    que.push_front(1);
    visited[1] = true;
    let mut cnt = 1;
    while let Some(a) = que.pop_back() {
        for &b in edges[a].iter() {
            if a == 1 && cnt == 2 { continue; }
            if cnt == n {
                println!("Yes");
                return;
            }
            if visited[b] { continue; }
            que.push_front(b);
            visited[b] = true;
            cnt += 1;
        }
    }
    println!("No");
}