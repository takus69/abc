use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut s = s;
    s.sort();
    let min_s: String = s.iter().collect();
    let ss:Result<i32, _> = min_s.parse();
    let min_s = min_s.parse::<i64>().unwrap_or(-1);
    s.sort_by(|a, b| b.cmp(&a));
    let max_s: String = s.iter().collect();
    let max_s = max_s.parse::<i64>().unwrap_or(-1);
    // println!("{} {}", min_s, max_s);

    let mut cnt = HashMap::new();
    for si in s {
        let mut tmp = 0;
        if cnt.contains_key(&si) { tmp = *cnt.get(&si).unwrap() }
        cnt.insert(si, tmp+1);
    }

    let min_n = (min_s as f64).sqrt() as u64;
    let max_n = (max_s as f64).sqrt() as u64;
    // println!("{} {}", min_n, max_n);

    let mut candidates = Vec::new();
    for i in min_n..=max_n {
        // println!("{}", i*i);
        candidates.push(i*i);
    }
    // println!("candidates: {}", candidates.len());

    let mut ans = 0;
    for can in candidates {
        let can = can.to_string();
        let can = can.chars();
        let mut cnt2 = HashMap::new();
        for si in can {
            let mut tmp = 0;
            if cnt2.contains_key(&si) { tmp = *cnt2.get(&si).unwrap() }
            cnt2.insert(si, tmp+1);
        }

        let mut flg = true;
        for (k, v) in cnt.iter() {
            if k == &'0' { continue; }
            if cnt2.contains_key(k) && cnt2[k] == cnt[k] {
                continue;
            } else {
                flg = false;
                break;
            }
        }
        for (k, v) in cnt2.iter() {
            if k == &'0' { continue; }
            if cnt.contains_key(k) && cnt2[k] == cnt[k] {
                continue;
            } else {
                flg = false;
                break;
            }
        }
        if flg { ans += 1; }
    }

    println!("{}", ans);
}