use proconio::input;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for (i, &ai) in a.iter().enumerate() {
        if map.contains_key(&ai) {
            let pre_i = map.get(&ai).unwrap();
            heap.push(Reverse(i-pre_i+1));
        }
        map.insert(ai, i);
    }
    if let Some(Reverse(ans)) = heap.pop() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}