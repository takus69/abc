use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(isize, isize); n],
    }
    let mut dp: Vec<Vec<isize>> = vec![vec![isize::MIN; 2*k+1]; n];
    dp[0][0] = ab[0].0;
    dp[0][1] = ab[0].1;
    for i in 0..(n-1) {
        let (a, b) = ab[i+1];
        for (j, &v) in dp[i].clone().iter().enumerate() {
            if v == isize::MIN { continue; }
            if j%2==0 {
                dp[i+1][j] = dp[i+1][j].max(dp[i][j]+a);
                if j < 2*k {
                    dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]+b);
                }
            } else {
                dp[i+1][j] = dp[i+1][j].max(dp[i][j]+b);
                if j < 2*k {
                    dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]+a);
                }
            }
        }
    }
    println!("{}", dp[n-1].iter().max().unwrap());
}