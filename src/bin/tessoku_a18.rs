use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; s+1]; n+1];
    dp[0][0] = 0;
    for (i, &ai) in a.iter().enumerate() {
        for j in 0..(s+1) {
            if dp[i][j] != usize::MAX {
                dp[i+1][j] = dp[i+1][j].min(dp[i][j]);
                if j+ai <= s {
                    dp[i+1][j+ai] = dp[i+1][j+ai].min(dp[i][j]+ai);
                }
            }
        }
    }
    for i in 0..=n {
        if dp[i][s] != usize::MAX {
            println!("Yes");
            return;
        }
    }
    println!("No");
}