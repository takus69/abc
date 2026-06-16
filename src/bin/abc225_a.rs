use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    if s[0]==s[1] && s[1]==s[2] {
        println!("1");
    } else if s[0]==s[1] || s[1]==s[2] {
        println!("3");
    } else {
        println!("6");
    }
}