use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut r: [usize; n],
        }
        let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
        for (i, &ri) in r.iter().enumerate() {
            heap.push((Reverse(ri), i));
        }
        let mut done: Vec<bool> = vec![false; n+1];
        let mut ans = 0;
        while let Some((Reverse(ri), i)) = heap.pop() {
            // println!("ri: {}, i: {}, r: {:?}", ri, i, r);
            if done[i] { continue; }
            done[i] = true;
            if i > 0 && !done[i-1] {
                let diff = r[i-1] - ri;
                ans += if diff > 1 {
                    r[i-1] -= diff - 1;
                    heap.push((Reverse(r[i-1]), i-1));
                    diff - 1
                } else { 0 };
            }
            if i < n-1 && !done[i+1] {
                let diff = r[i+1] - ri;
                ans += if diff > 1 {
                    r[i+1] -= diff - 1;
                    heap.push((Reverse(r[i+1]), i+1));
                    diff - 1
                } else { 0 };
            }
        }
        println!("{}", ans);
    }
}