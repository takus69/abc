use proconio::{input, marker::Chars};
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            s: Chars,
        }
        let mut cnt: HashMap<char, usize> = HashMap::new();
        for &si in &s {
            let e = cnt.entry(si).or_insert(0);
            *e += 1;
        }
        let mut heap: BinaryHeap<(usize, char)> = BinaryHeap::new();
        for (&si, &c) in cnt.iter() {
            heap.push((c, si));
        }

        let mut flg = true;
        let mut ans: Vec<char> = Vec::new();
        // 二回pop
        while let Some((c1, si1)) = heap.pop() {
            if let Some((c2, si2)) = heap.pop() {
                ans.push(si1);
                if c1 > 1 {
                    heap.push((c1-1, si1));
                }
                ans.push(si2);
                if c2 > 1 {
                    heap.push((c2-1, si2));
                }
            } else {
                if c1 > 1 {
                    flg = false;
                    break;
                } else if c1 == 1 {
                    ans.push(si1);
                }
            }
        }

        if flg {
            println!("Yes");
            println!("{}", ans.iter().join(""));
        } else {
            println!("No");
        }
    }

}