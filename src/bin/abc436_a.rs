use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }
    s.reverse();
    for _ in s.len()..n {
        s.push('o');
    }
    s.reverse();
    println!("{}", s.iter().join(""));
}