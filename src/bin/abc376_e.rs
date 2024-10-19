use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        t: usize,
    }
    for i in 0..t {
        input! {
            n: usize,
            k: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        }
        let mut pairs: Vec<(usize, usize)> = a.iter().cloned().zip(b.iter().cloned()).collect();
        pairs.sort_by_key(|&(a, _)| a);
        let (a, b): (Vec<usize>, Vec<usize>) = pairs.iter().cloned().unzip();
        let mut heap = BinaryHeap::new();
        let mut max_a = a[k-1];
        let mut sum_b = 0;
        for i in 0..k {
            heap.push(b[i]);
            sum_b += b[i];
        }
        let mut ans = max_a * sum_b;
        for i in k..n {
            heap.push(b[i]);
            let rm_b = heap.pop().unwrap();
            sum_b += b[i];
            sum_b -= rm_b;
            max_a = a[i];
            ans = ans.min(max_a * sum_b);
        }
        println!("{}", ans);
    }
}