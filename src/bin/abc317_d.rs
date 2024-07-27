use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }
    let sum_z: usize = xyz.iter().map(|x| x.2).sum();
    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MAX; sum_z+1]; n+1];
    dp[0][0] = 0;
    for (i, (xi, yi, zi)) in xyz.iter().enumerate() {
        for z in 0..dp[i+1].len() {
            let xy = dp[i][z];
            if xy != i64::MAX {
                dp[i+1][z] = dp[i+1][z].min(xy);
                if yi > xi {
                    dp[i+1][z+zi] = dp[i+1][z+zi].min(xy + (yi-xi) as i64 /2+1);
                } else {
                    dp[i+1][z+zi] = dp[i+1][z+zi].min(xy);
                }
            }
        }
    }
    let mut ans = i64::MAX;
    for z in (sum_z/2+1)..=sum_z {
        ans = ans.min(dp[n][z]);
    }
    println!("{}", ans);
}