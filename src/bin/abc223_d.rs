use proconio::input;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut indegree: Vec<usize> = vec![0; n+1];
    for &(a, b) in &ab {
        edge[a].push(b);
        indegree[b] += 1;
    }

    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for i in 1..=n {
        if indegree[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut ans: Vec<usize> = Vec::new();
    while let Some(Reverse(i)) = heap.pop() {
        ans.push(i);
        for &j in edge[i].iter() {
            indegree[j] -= 1;
            if indegree[j] == 0 {
                heap.push(Reverse(j));
            }
        }
    }

    if ans.len() == n {
        println!("{}", ans.iter().join(" "));
    } else {
        println!("-1");
    }
}