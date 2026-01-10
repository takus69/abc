use proconio::input;
use std::collections::{BinaryHeap, HashSet, BTreeSet};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: isize,
        x: usize,
        mut a: [isize; n],
    }
    a.sort();a.reverse();
    let mut heap: BinaryHeap<(isize, Vec<usize>)> = BinaryHeap::new();
    let mut added: HashSet<Vec<usize>> = HashSet::new();
    let mut cand: Vec<usize> = vec![0; n];
    cand[0] = k as usize;
    heap.push((a[0]*k, cand.clone()));
    added.insert(cand);
    let mut ans: Vec<isize> = Vec::new();
    while ans.len() < x {
        let (v, cand) = heap.pop().unwrap();
        ans.push(v);
        for i in 0..(n-1) {
            if cand[i] == 0 { continue; }
            let mut cand2 = cand.clone();
            cand2[i] -= 1;
            cand2[i+1] += 1;
            if added.contains(&cand2) { continue; }
            let v2 = v + a[i+1] - a[i];
            heap.push((v2, cand2.clone()));
            added.insert(cand2);
        }
    }
    println!("{}", ans.iter().join(" "));
}