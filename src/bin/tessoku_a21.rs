use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, isize); n],
    }
    let mut dp: Vec<Vec<isize>> = vec![vec![isize::MIN; n]; n];  // dp[l][r]: lからrまで残っている時の最大のスコア
    dp[0][n-1] = 0;
    for l in 0..(n-1) {
        for r in (0..(n-1)).rev() {  // r+2を取り除く
            if dp[l][r+1] == isize::MIN { continue; }
            let (p, a) = pa[r+1];
            let tmp = dp[l][r+1] + if l < p && p < r+2 { a } else { 0 };
            dp[l][r] = dp[l][r].max(tmp);
        }
        // lを取り除く
        let (p, a) = pa[l];
        for r in 0..n {
            dp[l+1][r] = dp[l][r] + if l < p && p < r+2 { a } else { 0 };
        }
    }

    let mut ans = 0;
    // println!("{:?}", dp);
    for i in 0..n {
        ans = ans.max(dp[i][i]);
    }
    println!("{}", ans);
}