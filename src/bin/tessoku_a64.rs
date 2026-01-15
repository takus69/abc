use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, isize); m],
    }

    let mut graph: Vec<Vec<(usize, isize)>> = vec![Vec::new(); n+1];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut dist: Vec<isize> = vec![isize::MAX; n+1];
    let mut heap: BinaryHeap<(Reverse<isize>, usize)> = BinaryHeap::new();
    let mut fixed: Vec<bool> = vec![false; n+1];
    dist[1] = 0;
    heap.push((Reverse(0), 1));
    fixed[1] = true;

    while let Some((Reverse(d), i)) = heap.pop() {
        fixed[i] = true;
        for &(j, c) in &graph[i] {
            if fixed[j] { continue; }
            if dist[j] > d+c {
                dist[j] = d+c;
                heap.push((Reverse(d+c), j));
            }
        }
    }

    for i in 1..=n {
        if dist[i] == isize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}