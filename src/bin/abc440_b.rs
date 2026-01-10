use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    for (i, &ti) in t.iter().enumerate() {
        heap.push((Reverse(ti), i+1));
    }
    let mut ans: Vec<usize> = Vec::new();
    for _ in 0..3 {
        let (Reverse(ti), i) = heap.pop().unwrap();
        ans.push(i);
    }
    println!("{}", ans.iter().join(" "));
}