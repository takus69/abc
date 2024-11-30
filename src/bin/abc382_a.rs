use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: Chars,
    }
    let mut cnt = 0;
    for &si in s.iter() {
        if si == '.' { cnt += 1; }
    }
    println!("{}", cnt + d);
}