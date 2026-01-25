use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    for &si in s.iter() {
        if si == 'i' || si == 'j' {
            ans += 1;
        }
    }
    println!("{}", ans);
}