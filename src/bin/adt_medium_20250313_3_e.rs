use proconio::input;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut ab: [(usize, usize); n],
    }
    let mut iab: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut abi: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    let mut bai: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        iab.insert(i, (a, b));
        abi.push((Reverse(a), b, i));
        bai.push((Reverse(b), a, i));
    }
    let mut new_a = 0;
    let mut pre_a = 0;
    while !abi.is_empty() {
        let (Reverse(a), b, i) = abi.pop().unwrap();
        if pre_a != a { new_a += 1; }
        iab.insert(i, (new_a, b));
        pre_a = a;
    }
    let mut new_b = 0;
    let mut pre_b = 0;
    while !bai.is_empty() {
        let (Reverse(b), a, i) = bai.pop().unwrap();
        let &(a, _) = iab.get(&i).unwrap();
        if pre_b != b { new_b += 1; }
        iab.insert(i, (a, new_b));
        pre_b = b;
    }
    
    for i in 0..n {
        let (a, b) = iab.get(&i).unwrap();
        println!("{} {}", a, b);
    }
}