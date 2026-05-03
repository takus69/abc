use proconio::{input, marker::Chars};
use std::collections::HashSet;

const MOD: usize = 998244353;

fn main() {
    input! {
        s: Chars,
    }
    fn ch(x: char) -> usize {
        match x {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => usize::MAX,
        }
    }
    let n = s.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]];
    for i in 0..n {
        let mut tmp: Vec<usize> = dp[i].clone();
        let si = s[i];
        tmp[ch(si)] = (dp[i][0] + dp[i][1] + dp[i][2] + 1)%MOD;
        dp.push(tmp);
    }
    println!("{}", (dp[n][0]+dp[n][1]+dp[n][2])%MOD);
}