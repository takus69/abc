use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    let mut flg = true;
    for &si in s.iter() {
        if flg && si!='#' {
            ans.push('o');
            flg = false;
        } else if si == '#' {
            ans.push(si);
            flg = true;
        } else {
            ans.push(si);
        }
    }
    println!("{}", ans.iter().join(""));
}