use std::arch::x86_64;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let mut x = s.iter().position(|&x| x == 'A').unwrap();
    for si in "BCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>() {
        let i = s.iter().position(|&x| x == si).unwrap();
        ans += x.abs_diff(i);
        x = i;
    }
    println!("{}", ans);
}