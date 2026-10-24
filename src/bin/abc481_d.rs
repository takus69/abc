use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        ab: [(usize, usize); m],
    }
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in &ab {
        graph[a].push(b);
    }
    for i in 1..=n {
        graph[i].sort();
    }
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut dist: Vec<usize> = vec![usize::MAX; n+1];
    heap.push((Reverse(0), s));
    dist[s] = 0;
    while let Some((Reverse(d), a)) = heap.pop() {
        if graph[a].len() > 0 {
            let b = graph[a][0];
            if dist[b] > d {
                heap.push((Reverse(d), b));
                dist[b] = d;
            }
            for &b in graph[a].iter().skip(1) {
                if dist[b] <= d+1 { continue; }
                heap.push((Reverse(d+1), b));
                dist[b] = d+1;
            }
        }
    }
    if dist[t] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[t]);
    }
}