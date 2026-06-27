use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            x: [isize; n],
            y: [isize; n-1],
        }
        let mut dp: Vec<isize> = if s[0]=='S' { vec![0, -x[0]] } else { vec![-x[0], 0] };  // 0: S, 1: Rで終わった時の最大
        for i in 1..n {
            let mut next_dp = vec![0, 0];
            if s[i] == 'S' {
                next_dp[0] = (dp[1]+y[i-1]).max(dp[0]);  // 変更しない場合
                next_dp[1] = dp[0].max(dp[1])-x[i];  // 変更する場合
            } else {
                next_dp[0] = (dp[1]+y[i-1]).max(dp[0])-x[i];  // 変更する場合
                next_dp[1] = dp[1].max(dp[0]);  // 変更しない場合
            }
            dp = next_dp;
        }
        println!("{}", dp[0].max(dp[1]));
    }
}