use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, usize); m],
    }
    let mut link: Vec<Vec<(usize, usize)>> = vec![vec![]; n+1];
    for &(u, v, w) in uvw.iter() {
        link[u].push((v, w));
        link[v].push((u, w));
    }
    let mut que: VecDeque<(usize, usize, Vec<bool>)> = VecDeque::new();  // (次の頂点, XOR値, visited)
    let mut ans = usize::MAX;
    let mut visited = vec![false; n+1];
    visited[1] = true;
    que.push_front((1, 0, visited));
    while !que.is_empty() {
        let (u, xor, visited) = que.pop_back().unwrap();
        for &(v, w) in link[u].iter() {
            if visited[v] { continue; }
            let xor2 = xor ^ w;
            let mut visited2 = visited.clone();
            visited2[v] = true;
            if v == n {
                ans = ans.min(xor2);
                continue;
            }
            que.push_front((v, xor2, visited2));
        }
    }
    println!("{}", ans);
}