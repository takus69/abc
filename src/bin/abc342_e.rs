use proconio::input;
use std::{collections::BinaryHeap, fmt::Binary};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut edge: Vec<Vec<(usize, usize, usize, usize, usize)>> = vec![Vec::new(); n+1];
    for _ in 0..m {
        input! {
            l: usize,
            d: usize,
            k: usize,
            c: usize,
            a: usize,
            b: usize,
        }
        edge[b].push((a, l, d, k, c));
    }
    
    let mut ans: Vec<usize> = vec![0; n];
    let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    heap.push((usize::MAX, n));
    while let Some((t, b)) = heap.pop() {
        visited[b] = true;
        // println!("start b: {}, t: {}", b, t);
        for &(a, l, d, k, c) in edge[b].iter() {
            // println!("a: {}, l: {}, d: {}, k: {}, c: {}, t: {}", a, l, d, k, c, t);
            if visited[a] { continue; }
            let s = if t == usize::MAX {
                l+(k-1)*d
            } else {
                if t < l+c { continue; }
                let mut x = t-(l+c);
                x /= d;
                if x >= k { x = k-1; }
                l+x*d
            };
            ans[a] = ans[a].max(s);
            heap.push((s, a));
            // println!("a: {}, s: {}", a, s);
        }
    }

    for i in 1..n {
        if ans[i] == 0 {
            println!("Unreachable");
        } else {
            println!("{}", ans[i]);
        }
    }
}
