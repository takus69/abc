use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        x: usize,
        q: usize,
    }

    let mut large: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut small: BinaryHeap<usize> = BinaryHeap::new();
    let mut median = x;

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }

        if a > median && b > median {
            large.push(Reverse(a));
            large.push(Reverse(b));
            let Reverse(c) = large.pop().unwrap();
            small.push(median);
            median = c;
        } else if median > a && median > b {
            small.push(a);
            small.push(b);
            let c = small.pop().unwrap();
            large.push(Reverse(median));
            median = c;
        } else if a > b {
            large.push(Reverse(a));
            small.push(b);
        } else {
            large.push(Reverse(b));
            small.push(a);
        }

        println!("{}", median);
    }
}