use proconio::{input, marker::Chars};
use std::collections::HashMap;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let max = 2usize.pow(n as u32);
    let mut dp: Vec<Vec<usize>> = vec![vec![0; max]; n+1];
    dp[0][max-1] = 1;
    for i in 0..n {
        for j in 0..max {
            if dp[i][j] == 0 { continue; }
            let mut pre = '?';
            for k in 0..n {
                if (j >> k)&1 == 0 { continue; }
                if t[k] == pre { continue; }
                dp[i+1][j-(1<<k)] += dp[i][j];
                dp[i+1][j-(1<<k)] %= MOD;
                pre = t[k];
            }

        }
    }
    println!("{}", dp[n].iter().sum::<usize>());
}