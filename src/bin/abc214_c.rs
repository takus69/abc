use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut ans: Vec<usize> = t.clone();
    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    for (i, &ti) in t.iter().enumerate() {
        heap.push(Reverse((ti, i)));
    }
    while let Some(Reverse((ti, i))) = heap.pop() {
        if ans[i] < ti { continue; }
        let next_i = (i+1)%n;
        let next_ti = ti+s[i];
        if ans[next_i] > next_ti {
            ans[next_i] = next_ti;
            heap.push(Reverse((next_ti, next_i)));
        }
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}