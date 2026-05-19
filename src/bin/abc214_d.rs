use proconio::input;
use ac_library::Dsu;
use std::collections::BinaryHeap;
use std::cmp:: Reverse;

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, usize); n-1],
    }
    let mut ans = 0;
    let mut dsu = Dsu::new(n+1);
    let mut heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    for &(u, v, w) in &uvw {
        heap.push(Reverse((w, u, v)));
    }
    while let Some(Reverse((w, u, v))) = heap.pop() {
        ans += w*dsu.size(u)*dsu.size(v);
        dsu.merge(u, v);
    }

    println!("{}", ans);
}