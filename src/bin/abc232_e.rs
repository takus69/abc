use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        (x1, y1, x2, y2): (usize, usize, usize, usize),
    }
    const MOD: usize = 998244353;
    let mut dp: Vec<Vec<usize>> = vec![vec![0, 0, 0, 0]];  // i回目の操作でいる場所の通り、(x2, y2)、x2行、y2列、その他の場所
    if (x1, y1) == (x2, y2) {
        dp[0][0] = 1;
    } else if x1 == x2 {
        dp[0][1] = 1;
    } else if y1 == y2 {
        dp[0][2] = 1;
    } else {
        dp[0][3] = 1;
    }
    for i in 0..k {
        // println!("dp: {:?}", dp[i]);
        dp.push(vec![0, 0, 0, 0]);
        dp[i+1][0] = dp[i][1] + dp[i][2];
        dp[i+1][1] = dp[i][0]*(w-1) + dp[i][1]*(w-2) + dp[i][3];
        dp[i+1][2] = dp[i][0]*(h-1) + dp[i][2]*(h-2) + dp[i][3];
        dp[i+1][3] = dp[i][1]*(h-1) + dp[i][2]*(w-1) + dp[i][3]*(h-2) + dp[i][3]*(w-2);
        dp[i+1][0] %= MOD;
        dp[i+1][1] %= MOD;
        dp[i+1][2] %= MOD;
        dp[i+1][3] %= MOD;
    }
    // println!("dp: {:?}", dp[k]);
    println!("{}", dp[k][0]);
}