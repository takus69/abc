use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n],
    }
    td.sort_by(|a, b| a.1.cmp(&b.1));
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; 1441]];
    dp[0][0] = 0;
    for i in 0..n {
        // println!("i: {}", i);
        let mut next_dp: Vec<usize> = dp[i].clone();
        let (t, d) = td[i];
        for (j, &c) in dp[i].iter().enumerate() {
            if c == usize::MAX { continue; }
            if j+t > d { continue; }
            if next_dp[j+t] == usize::MAX {
                next_dp[j+t] = c+1;
                // println!("j+t: {}, next_dp: {}", j+t, next_dp[j+t]);
            } else {
                next_dp[j+t] = next_dp[j+t].max(c+1);
                // println!("j+t: {}, next_dp: {}", j+t, next_dp[j+t]);
            }
        }
        dp.push(next_dp);
    }
    let mut ans = 0;
    for &c in &dp[n] {
        if c == usize::MAX { continue; }
        ans = ans.max(c);
    }
    println!("{}", ans);
}