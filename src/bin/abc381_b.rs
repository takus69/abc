use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut ans = "Yes";
    if s.len() % 2 != 0 { ans = "No"; }
    for i in 0..(s.len()/2) {
        if s[2*i] != s[2*i+1] { ans = "No"; break; }
        let si = s[2*i];
        *map.entry(si).or_insert(0) += 2;
    }
    for (k, &v) in map.iter() {
        if v != 2 {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}