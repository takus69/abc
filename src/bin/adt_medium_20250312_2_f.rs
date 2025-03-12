use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: usize = 0;
    let mut base = 1;
    for (i, &si) in s.iter().rev().enumerate() {
        let n = si as usize - 'A' as usize + 1;
        ans += base * n;
        base *= 26;
    }
    println!("{}", ans);
}