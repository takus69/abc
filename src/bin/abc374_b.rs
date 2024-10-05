use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let len = s.len().min(t.len());
    let mut ans = if s.len() != t.len() { len+1 } else { 0 };
    for i in 0..len {
        if s[i] != t[i] {
            ans = i+1;
            break;
        }
    }
    println!("{}", ans);
}
