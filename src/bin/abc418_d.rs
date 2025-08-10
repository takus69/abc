use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0, 0]; n+1];
    for (i, &ti) in t.iter().enumerate() {
        if ti == '1' {
            dp[i+1][0] = dp[i][0];
            dp[i+1][1] = dp[i][1] + 1;
        } else {
            dp[i+1][0] = dp[i][1] + 1;
            dp[i+1][1] = dp[i][0];
        }
    }
    let mut ans = 0;
    for d in dp.iter() {
        ans += d[1];
    }
    println!("{}", ans);
}