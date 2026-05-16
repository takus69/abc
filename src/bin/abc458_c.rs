use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let n = s.len();
    for (i, &si) in s.iter().enumerate() {
        if si == 'C' {
            ans += (i+1).min(n-i);
        }
    }

    println!("{}", ans);
}