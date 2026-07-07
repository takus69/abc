use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let a = s[0] as usize - '0' as usize;
    let b = s[2] as usize - '0' as usize;
    println!("{}", a*b);
}