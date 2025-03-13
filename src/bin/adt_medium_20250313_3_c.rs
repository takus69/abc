use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut ans: Vec<Vec<char>> = Vec::new();
    let mut max_s = 0;
    for si in s.iter() {
        max_s = max_s.max(si.len());
        while ans.len() < max_s { ans.push(Vec::new()); }
        for j in 0..max_s {
            if j >= si.len() {
                ans[j].push('*');
            } else {
                ans[j].push(si[j]);
            }
        }
    }
    for a in ans.iter() {
        println!("{}", a.iter().rev().join(""));
    }
}