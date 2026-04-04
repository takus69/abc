use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
        m: usize,
        s: [Chars; m],
    }

    let mut map: HashMap<(usize, usize), HashSet<char>> = HashMap::new();
    for i in 0..m {
        for &(a, b) in ab.iter() {
            let e = map.entry((a, b)).or_insert(HashSet::new());
            if a == s[i].len() {
                e.insert(s[i][b-1]);
            }
        }
    }
    for si in s.iter() {
        if si.len() != n {
            println!("No");
            continue;
        }
        let mut all_ok = true;
        for j in 0..n {
            let (a, b) = ab[j];
            // sのうちaと長さが一致する文字列
            if !map.get(&(a, b)).unwrap().contains(&si[j]) {
                all_ok = false;
                break;
            }
        }
        if all_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}