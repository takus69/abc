use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; s.len()+1]; t.len()+1];
    for i in 0..=t.len() {
        for j in 0..=s.len() {
            if i < t.len() && j < s.len() && t[i] == s[j] {
                dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]+1);
            }
            if i < t.len() {
                dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            }
            if j < s.len() {
                dp[i][j+1] = dp[i][j+1].max(dp[i][j]);
            }
        }
    }
    dp[t.len()][s.len()] = dp[t.len()][s.len()].max(dp[t.len()-1][s.len()]).max(dp[t.len()][s.len()-1]);
    println!("{}", dp[t.len()][s.len()]);
}