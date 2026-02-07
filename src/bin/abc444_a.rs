use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: Chars,
    }
    let s: HashSet<&char> = n.iter().collect();
    if s.len() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}