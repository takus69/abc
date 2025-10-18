use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }
    let mut ans = "Yes";
    for i in (l-1)..r {
        if s[i] != 'o' {
            ans = "No";
        }
    }
    println!("{}", ans);
}