use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    for i in 0..n {
        let (l, r) = lr[i];
        heap.push(Reverse((r, i)));
    }
    let mut cnt = 0;
    let mut pre = 0;
    for i in 0..n {
        let (l, mut r) = lr[i];
        while let Some(Reverse((r2, ri))) = heap.peek() {
            if *ri < i { heap.pop(); } else { r = *r2; break; }
        }
        cnt += (l-pre) * (m-r+1);
        pre = l;
    }
    println!("{}", m+m*(m-1)/2 - cnt);
}