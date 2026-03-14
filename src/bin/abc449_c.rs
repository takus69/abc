use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }
    let mut cnt: HashMap<char, usize> = HashMap::new();
    for i in l..=r {
        let si = s[i];
        let e = cnt.entry(si).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        let si = s[i];
        let e = cnt.entry(si).or_insert(0);
        ans += *e;
        if i+l < n {
            let sl = s[i+l];
            let e = cnt.entry(sl).or_insert(0);
            *e -= 1;
        }
        if i+r+1 < n {
            let sr = s[i+r+1];
            let e = cnt.entry(sr).or_insert(0);
            *e += 1;
        }
    }
    println!("{}", ans);
}