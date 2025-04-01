use proconio::{input, marker::Chars};
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut set: HashSet<String> = HashSet::new();
    let mut cnt = 0;
    for _ in 0..n {
        input! {
            s: Chars,
        }
        let mut s2 = s.clone();
        s2.reverse();
        let s = s.iter().join("");
        let s2 = s2.iter().join("");
        if !set.contains(&s) && s == s2 {
            cnt += 1;
        } else {
            set.insert(s2);
        }
        set.insert(s);
    }
    cnt += set.len();
    println!("{}", cnt/2);
}