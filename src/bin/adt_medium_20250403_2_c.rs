use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        t: Chars,
    }
    let mut ans = "Yes";
    if t.len()%2 == 1 { ans = "No"; }
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..t.len()/2 {
        if set.contains(&t[2*i]) { ans = "No";break; }
        if t[2*i] != t[2*i+1] { ans = "No";break; }
        set.insert(t[2*i]);
    }
    println!("{}", ans);
}