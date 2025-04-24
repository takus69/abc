use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let mut set1: HashSet<char> = HashSet::new();
    let mut set2: HashSet<char> = HashSet::new();
    let mut ans = "Yes";
    for &si in s.iter() {
        if set1.contains(&si) || set2.contains(&si) {
            ans = "No";
        }
        if si.is_ascii_lowercase() {
            set1.insert(si);
        } else {
            set2.insert(si);
        }
    }

    if set1.is_empty() || set2.is_empty() {
        ans = "No";
    }
    println!("{}", ans);
}