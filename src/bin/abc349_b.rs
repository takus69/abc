use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut map = HashMap::new();
    let mut v_cnt = vec![0; s.len()+1];
    for si in s {
        let cnt = map.entry(si).or_insert(0);
        *cnt += 1;
    }
    // println!("{:?}", map);
    for (k, v) in map {
        v_cnt[v as usize] += 1;
    }
    // println!("{:?}", v_cnt);
    let mut ans = String::from("Yes");
    for v in v_cnt {
        if v != 0 && v != 2 {
            ans = String::from("No");
        }
    }
    println!("{}", ans);
}