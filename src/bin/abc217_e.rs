use proconio::input;
use std::collections::{BinaryHeap, VecDeque};
use std::cmp:: Reverse;

fn main() {
    input! {
        q: usize,
    }
    let mut que: VecDeque<usize> = VecDeque::new();
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
                que.push_back(x);
            },
            2 => {
                if !heap.is_empty() {
                    let Reverse(x) = heap.pop().unwrap();
                    println!("{}", x);
                } else {
                    let x = que.pop_front().unwrap();
                    println!("{}", x);
                }
            },
            _ => {
                while let Some(x) = que.pop_front() {
                    heap.push(Reverse(x));
                }
            },
        }
    }
    
}