use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
        v: isize,
    }
    let mut heap: BinaryHeap<isize> = BinaryHeap::new();
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                t: isize,
                w: isize,
            }
            heap.push(w-t);
        } else {
            input! {
                t: isize,
            }
            if heap.is_empty() {
                println!("-1");
            } else {
                let vv = heap.pop().unwrap()+t;
                if vv > v {
                    println!("{}", v);
                } else {
                    println!("{}", vv);
                }
            }
        }
    }
}