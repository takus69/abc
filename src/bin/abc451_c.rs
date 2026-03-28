use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: usize,
    }
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for _ in 0..q {
        input! {
            c: usize,
            h :usize,
        }
        if c == 1 {
            heap.push(Reverse(h));
        } else {
            while let Some(Reverse(h2)) = heap.pop() {
                if h2 > h {
                    heap.push(Reverse(h2));
                    break;
                }
            }
        }
        println!("{}", heap.len());
    }
}