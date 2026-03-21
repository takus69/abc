use proconio::input;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut min: usize = usize::MAX;
    for &ai in a.iter() {
        heap.push(Reverse(ai));
        min = min.min(ai);
    }

    for _ in 0..q {
        input! {
            k: usize,
            b: [usize; k],
        }
        let mut ans = min;
        let mut set: HashSet<usize> = HashSet::new();
        let mut tmp: Vec<Reverse<usize>> = Vec::new();
        for &bi in b.iter() {
            let ai = a[bi-1];
            if ans == ai {
                let t = heap.pop().unwrap();
                tmp.push(t);
                while let Some(Reverse(bi)) = heap.peek() {
                    if set.contains(bi) {
                        let t = heap.pop().unwrap();
                        tmp.push(t);
                    } else {
                        ans = *bi;
                        break;
                    }
                }
            } else {
                set.insert(ai);
            }
        }
        println!("{}", ans);
        for t in tmp {
            heap.push(t);
        }
    }
}