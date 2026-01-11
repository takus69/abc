use proconio::input;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xyc: [(usize, usize, usize); m],
    }
    for i in 0..m {
        xyc[i].0 -= 1;
        xyc[i].1 -= 1;
    }
    let mut x_link: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut y_link: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, &(x, y, c)) in xyc.iter().enumerate() {
        x_link[x].push(i);
        y_link[y].push(i);
    }
    let mut y_edge: Vec<Vec<usize>> = vec![Vec::new(); m];
    for (i, &(x, y, c)) in xyc.iter().enumerate() {
        y_edge[i].extend(&x_link[y]);
    }

    // ダイクストラ
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; m];
    let mut cost: Vec<usize> = vec![usize::MAX; m];
    heap.push((Reverse(xyc[0].2), 0));
    visited[0] = true;
    cost[0] = xyc[0].2;
    while let Some((Reverse(c), i)) = heap.pop() {
        for &j in y_edge[i].iter() {
            if visited[j] { continue; }
            let next_c = c + xyc[j].2;
            heap.push((Reverse(next_c), j));
            visited[j] = true;
            cost[j] = next_c;
        }
    }
    let mut ans = usize::MAX;
    for i in 0..m {
        if xyc[0].0 != xyc[i].1 || cost[i] == usize::MAX { continue; }
        ans = ans.min(cost[i]);
    }
    if ans != usize::MAX {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}