use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut stronger: Vec<bool> = vec![false; n];
    for (a, b) in ab.iter() {
        stronger[b-1] = true;
    }
    let mut ans = -1;
    for i in 0..n {
        if !stronger[i] {
            if ans > 0 {
                ans = -1;
                break;
            }
            ans = i as i32 +1;
        }
    }
    println!("{}", ans);
}