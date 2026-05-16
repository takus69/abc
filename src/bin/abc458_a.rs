use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    println!("{}", s[n..s.len()-n].iter().join(""));
}