use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }
    let n = s.len();
    let mut ans = String::new();
    for i in 0..n {
        if l <= i+1 && i+1 <= r {
            ans.push(s[r-(i+1-l)-1]);
        } else {
            ans.push(s[i]);
        }
    }
    println!("{}", ans);
}