use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    }
    n.reverse();
    let mut ans = 0;
    for i in 0..n.len() {
        if n[i] == '0' { continue; }
        ans += 1 << i;
    }
    println!("{}", ans);
}