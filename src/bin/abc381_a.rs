use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = "Yes";
    if n % 2 == 0 {
        ans = "No";
    }
    for i in 0..n {
        if i+1 < (n+1) / 2 {
            if s[i] != '1' {
                ans = "No";
            }
        } else if i+1 == (n+1) / 2 {
            if s[i] != '/' {
                ans = "No";
            }
        } else {
            if s[i] != '2' {
                ans = "No";
            }
        }
    }
    println!("{}", ans);
}