use proconio::input;

fn main() {
    input! {
        t: String,
        n: usize,
    }
    let mut a: Vec<usize> = Vec::new();
    let mut s: Vec<Vec<String>> = Vec::new();
    for _ in 0..n {
        input! {
            ai: usize,
            si: [String; ai],
        }
        a.push(ai);
        s.push(si);
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; t.len()+1]; n+1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][0] = dp[i-1][0];
        for si in s[i-1].iter() {
            for j in 0..=t.len() {
                dp[i][j] = dp[i][j].min(dp[i-1][j]);
                if dp[i-1][j] != usize::MAX && j+si.len() <= t.len() && t[j..(j+si.len())] == si[0..si.len()] {
                    dp[i][j+si.len()] = dp[i][j+si.len()].min(dp[i-1][j]+1);
                }
            }
        }
    }
    let mut ans = usize::MAX;
    for i in 0..=n {
        if dp[i][t.len()] != usize::MAX {
            ans = ans.min(dp[i][t.len()]);
        }
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}