use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    let mut passed: Vec<bool> = vec![false; n];
    // 数学
    let mut heap: BinaryHeap<(usize, Reverse<usize>)> = BinaryHeap::new();
    for (i, &ai) in a.iter().enumerate() {
        heap.push((ai, Reverse(i)));
    }
    let mut cnt = 0;
    while let Some((_, Reverse(i))) = heap.pop() {
        if cnt < x {
            if !passed[i] {
                ans.push(i+1);
                passed[i] = true;
                cnt += 1;
            }
        } else {
            break;
        }
    }
    // 英語
    let mut heap: BinaryHeap<(usize, Reverse<usize>)> = BinaryHeap::new();
    for (i, &bi) in b.iter().enumerate() {
        heap.push((bi, Reverse(i)));
    }
    let mut cnt = 0;
    while let Some((_, Reverse(i))) = heap.pop() {
        if cnt < y {
            if !passed[i] {
                ans.push(i+1);
                passed[i] = true;
                cnt += 1;
            }
        } else {
            break;
        }
    }
    // 合計
    let mut heap: BinaryHeap<(usize, Reverse<usize>)> = BinaryHeap::new();
    for i in 0..n {
        heap.push((a[i]+b[i], Reverse(i)));
    }
    let mut cnt = 0;
    while let Some((_, Reverse(i))) = heap.pop() {
        if cnt < z {
            if !passed[i] {
                ans.push(i+1);
                passed[i] = true;
                cnt += 1;
            }
        } else {
            break;
        }
    }
    ans.sort();
    for a in ans.iter() {
        println!("{}", a);
    }
}