use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    for &si in s.iter() {
        let e = map.entry(si).or_insert(0);
        *e += 1;
    }
    for (&k, &v) in map.iter() {
        if v == 1 {
            println!("{}", k);
        }
    }
}