use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    if s[0] == *s.last().unwrap() {
        println!("Yes");
    } else {
        println!("No");
    }
}