use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        ab: [(isize, isize); n],
    }
    let mut pre_dp: Vec<isize> = vec![isize::MIN; h+1];
    let mut ans = 0;
    pre_dp[h] = m as isize;
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        let mut dp: Vec<isize> = vec![isize::MIN; h+1];
        let mut flg = false;
        for (hi, &mi) in pre_dp.iter().enumerate() {
            if mi == isize::MIN { continue; }
            if mi >= bi {
                dp[hi] = dp[hi].max(pre_dp[hi]-bi);
                flg = true;
            }
            if hi >= ai as usize {
                dp[hi-ai as usize] = dp[hi-ai as usize].max(pre_dp[hi]);
                flg = true;
            }
        }
        pre_dp = dp;
        if flg {
            ans = i+1
        }
    }
    println!("{}", ans);
}