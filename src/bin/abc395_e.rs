use proconio::input;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        uv: [(usize, usize); m],
    }
    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut link2: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(ui, vi) in uv.iter() {
        let e = link.entry(ui).or_insert(Vec::new());
        e.push(vi);
        let e = link2.entry(vi).or_insert(Vec::new());
        e.push(ui);
    }

    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    heap.push((Reverse(0), 1, 0));
    let mut ans0: Vec<usize> = vec![usize::MAX; n+1];
    let mut ans1: Vec<usize> = vec![usize::MAX; n+1];
    ans0[1] = 0;

    while !heap.is_empty() {
        let (Reverse(cost), u, flg) = heap.pop().unwrap();
        if link.contains_key(&u) {
            for &vi in link.get(&u).unwrap().iter() {
                let cost2 = if flg == 0 { 1 } else { x+1 };
                if cost+cost2 < ans0[vi] {
                    heap.push((Reverse(cost+cost2), vi, 0));
                    ans0[vi] = cost+cost2;
                }
            }
        }
        if link2.contains_key(&u) {
            for &vi in link2.get(&u).unwrap().iter() {
                let cost2 = if flg == 1 { 1 } else { x+1 };
                if cost+cost2 < ans1[vi] {
                    heap.push((Reverse(cost+cost2), vi, 1));
                    ans1[vi] = cost+cost2;
                }
            }
        }
    }

    println!("{}", ans0[n].min(ans1[n]));
}