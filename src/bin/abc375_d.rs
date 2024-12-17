use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let mut cnt: HashMap<char, usize> = HashMap::new();
    for ti in t.iter() {
        cnt.insert(*ti, 0);
        for si in s.iter() {
            if ti == si {
                let e = cnt.get_mut(ti).unwrap();
                *e += 1;
            }
        }
    }

    let mut ans = 0;
    for ti in t.iter() {
        let c = cnt.get(ti).unwrap();
        let mut i = 0;
        let mut cnt2 = 1;
        for si in s.iter() {
            if ti == si {
                ans += (c-i)*i*cnt2 - i;
                i += 1;
                cnt2 = 1;
            } else {
                cnt2 += 1;
            }
        }
    }
    println!("{}", ans);
}