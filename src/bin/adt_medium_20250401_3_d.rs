use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = 0;
    for i in 0..s.len().min(t.len()) {
        if s[i] != t[i] {
            ans = i+1;
            break;
        }
    }
    if ans == 0 && s.len() != t.len() {
        ans = s.len().min(t.len()) + 1;
    }
    println!("{}", ans);
}