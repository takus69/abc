use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; 1001]];
    dp[0][0] = 0;
    for i in 0..n {
        let mut next_dp = dp[i].clone();
        let ai = a[i];
        for j in 0..=1000 {
            if dp[i][j] == usize::MAX { continue; }
            if j + ai > 1000 { continue; }
            if dp[i][j] == 3 { continue; }
            next_dp[j+ai] = (if next_dp[j+ai] == usize::MAX { 0 } else { next_dp[j+ai] }).max(dp[i][j] + 1);
        }
        dp.push(next_dp);
    }
    if dp[n][1000] == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}