use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp: Vec<Vec<isize>> = vec![vec![isize::MIN; w+1]; n+1];
    dp[0][0] = 0;
    for (i, &(wi, vi)) in wv.iter().enumerate() {
        for j in 0..=w {
            if dp[i][j] == isize::MIN { continue; }
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            if j+wi > w { continue; }
            dp[i+1][j+wi] = dp[i+1][j+wi].max(dp[i][j]+vi as isize);
        }
    }
    let mut ans = 0;
    for i in 0..=n {
        for j in 0..=w {
            ans = ans.max(dp[i][j]);
        }
    }
    println!("{}", ans);
}