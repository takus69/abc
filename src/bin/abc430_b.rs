use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut ans: HashSet<Vec<Vec<char>>> = HashSet::new();
    for i in 0..=(n-m) {
        for j in 0..=(n-m) {
            let mut tmp: Vec<Vec<char>> = Vec::new();
            for k in 0..m {
                tmp.push(s[i+k][j..(j+m)].to_vec());
            }
            ans.insert(tmp.clone());
        }
    }

    println!("{}", ans.len());
}