use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut right = true;
    let mut l = 0;
    let mut r = n;
    let mut ans: Vec<usize> = vec![0; n];
    for (i, &si) in s.iter().enumerate().rev() {
        if si == 'o' {
            right = !right;
        }
        if right {
            ans[r-1] = i+1;
            r -= 1;
        } else {
            ans[l] = i+1;
            l += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}