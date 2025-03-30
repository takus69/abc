use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for (i, &pi) in p.iter().enumerate() {
        heap.push((pi, i));
    }
    let mut ans = vec![0; n];
    let mut r = 0;
    let mut pre_p = 0;
    let mut k = 1;
    while !heap.is_empty() {
        let (pi, i) = heap.pop().unwrap();
        if pre_p == pi {
            k += 1;
        } else {
            r += k;
            pre_p = pi;
            k = 1;
        }
        ans[i] = r;
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}