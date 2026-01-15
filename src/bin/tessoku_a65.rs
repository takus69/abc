use proconio::input;
use std::collections::{BinaryHeap, VecDeque};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    }
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for (i, &ai) in a.iter().enumerate() {
        graph[ai].push(i+2);
    }
    let mut order: Vec<(usize, usize)> = Vec::new();
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n+1];
    que.push_front(1);
    visited[1] = true;
    while let Some(a) = que.pop_back() {
        for &b in graph[a].iter() {
            if visited[b] { continue; }
            visited[b] = true;
            order.push((b, a));
            que.push_front(b);
        }
    }

    let mut member: Vec<usize> = vec![0; n+1];
    while let Some((a, p)) = order.pop() {
        member[p] += member[a]+1;
    }
    println!("{}", member.iter().skip(1).join(" "));
}