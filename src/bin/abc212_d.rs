use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: usize,
    }

    let mut heap: BinaryHeap<Reverse<isize>> = BinaryHeap::new();
    let mut stock: isize = 0;
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c==3 {
            let Reverse(x) = heap.pop().unwrap();
            println!("{}", x+stock);
        } else {
            input! {
                x: isize,
            }

            if c==1 {
                heap.push(Reverse(x-stock));
            } else {
                stock += x;
            }
        }
    }
}