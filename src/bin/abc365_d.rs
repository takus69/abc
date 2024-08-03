use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp: Vec<HashMap<char, usize>> = Vec::new();
    dp.push(HashMap::new());
    for c in ['R', 'P', 'S'] {
        dp[0].insert(c, 0);
    }
    for i in 0..n {
        let si = s[i];
        let dpi = dp[i].clone();
        let s_cnt = match dpi.get(&'S') { Some(cnt) => {*cnt}, _ => {0}};
        let r_cnt = match dpi.get(&'R') { Some(cnt) => {*cnt}, _ => {0}};
        let p_cnt = match dpi.get(&'P') { Some(cnt) => {*cnt}, _ => {0}};
        // println!("si: {}, {:?}, s: {}, r: {}, p: {}", si, dpi, s_cnt, r_cnt, p_cnt);
        dp.push(HashMap::new());
        for c in ['R', 'P', 'S'] {
            if c == 'R' {
                if si == 'R' {
                    dp[i+1].insert(c, s_cnt.max(p_cnt));
                } else if si == 'S' {
                    dp[i+1].insert(c, s_cnt.max(p_cnt)+1);
                }
            } else if c == 'P' {
                if si == 'P' {
                    dp[i+1].insert(c, s_cnt.max(r_cnt));
                } else if si == 'R' {
                    dp[i+1].insert(c, s_cnt.max(r_cnt)+1);
                }
            } else if c == 'S' {
                if si == 'S' {
                    dp[i+1].insert(c, p_cnt.max(r_cnt));
                } else if si == 'P' {
                    dp[i+1].insert(c, p_cnt.max(r_cnt)+1);
                }
            }
            // println!("c: {}, {:?}", c, dp);
        }
    }
    let s_cnt = match dp[n].get(&'S') { Some(cnt) => {*cnt}, _ => {0}};
    let r_cnt = match dp[n].get(&'R') { Some(cnt) => {*cnt}, _ => {0}};
    let p_cnt = match dp[n].get(&'P') { Some(cnt) => {*cnt}, _ => {0}};
    println!("{}", s_cnt.max(r_cnt).max(p_cnt));
}