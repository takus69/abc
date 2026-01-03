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
        }
        match c {
            1 => {
                input! {
                    x: usize,
                }
                heap.push(Reverse(x));
            },
            2 => {
                let Reverse(x) = heap.peek().unwrap();
                println!("{}", x);
            },
            3 => {
                heap.pop();
            },
            _ => {},
        }
    }
}