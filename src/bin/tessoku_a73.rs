use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut edge: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); n+1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        edge[a].push((b, c, d));
        edge[b].push((a, c, d));
    }
    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    heap.push((Reverse(0), 0, 1));
    while let Some((Reverse(dist), cnt, u)) = heap.pop() {
        if u == n {
            println!("{} {}", dist, cnt);
            break;
        }
        if visited[u] { continue; }
        for &(v, c, d) in edge[u].iter() {
            if visited[v] { continue; }
            heap.push((Reverse(dist+c), cnt+d, v));
        }
        visited[u] = true;
    }
}