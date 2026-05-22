use proconio::{input, marker::Chars};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }
    let mut ans: HashSet<Vec<char>> = HashSet::new();
    for perm in (0..s.len()).permutations(s.len()) {
        let mut tmp: Vec<char> = Vec::new();
        for &i in &perm {
            tmp.push(s[i]);
        }
        ans.insert(tmp);
    }
    let mut ans: Vec<Vec<char>> = ans.into_iter().collect();
    ans.sort();
    println!("{}", ans[k-1].iter().join(""));
}