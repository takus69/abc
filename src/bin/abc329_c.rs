use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut pre = '0';
    let mut cnt = 0;
    for si in s.iter() {
        if pre == *si {
            cnt += 1;
        } else {
            pre = *si;
            cnt = 1;
        }
        let e = map.entry(*si).or_insert(0);
        if *e < cnt {
            *e = cnt;
        }
    }
    let mut ans = 0;
    for (_, v) in map.iter() {
        ans += v;
    }
    println!("{}", ans);
}