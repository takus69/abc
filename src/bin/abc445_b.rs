use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let mut max_len = 0;
    for si in s.iter() {
        max_len = max_len.max(si.len());
    }

    for i in 0..n {
        let l = s[i].len();
        for _ in 0..((max_len-l)/2) {
            s[i].insert(0, '.');
            s[i].push('.');
        }
    }
    for si in s.iter() {
        println!("{}", si.iter().join(""));
    }
}