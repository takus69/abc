use proconio::{input, marker::Chars};
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    for &si in s.iter() {
        let e = map.entry(si).or_insert(0);
        *e += 1;
    }

    let mut max_cnt = 0;
    for (_, &cnt) in map.iter() {
        max_cnt = max_cnt.max(cnt);
    }

    let mut del: Vec<char> = Vec::new();
    for (&si, &cnt) in map.iter() {
        if cnt == max_cnt {
            del.push(si);
        }
    }

    let mut ans: Vec<char> = Vec::new();
    for &si in s.iter() {
        if !del.contains(&si) {
            ans.push(si);
        }
    }

    println!("{}", ans.iter().join(""));
}