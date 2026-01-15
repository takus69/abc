use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
    for &(a, b, c) in abc.iter() {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut fixed: Vec<bool> = vec![false; n+1];
    let mut ans = 0;
    heap.push((Reverse(0), 1));
    while let Some((Reverse(c), i)) = heap.pop() {
        if fixed[i] { continue; }
        fixed[i] = true;
        ans += c;
        for &(j, c2) in graph[i].iter() {
            if fixed[j] { continue; }
            heap.push((Reverse(c2), j));
        }
    }

    println!("{}", ans);
}