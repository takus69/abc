use proconio::{input, marker::Chars};

fn main() {
  input! {
    t: usize,
  }
  for _ in 0..t {
    input! {
      n: usize,
      s: Chars,
    }
    let mut cnt0: Vec<usize> = Vec::new();
    let mut cnt1: Vec<usize> = Vec::new();
    let mut first = true;
    let mut c0 = 0;
    let mut c1 = 0;
    for &si in s.iter() {
      if first && si == '0' { continue; }
      first = false;
      if si == '1' {
        c1 += 1;
        if c0 > 0 {
            cnt0.push(c0);
            c0 = 0;
        }
      } else {
        c0 += 1;
        if c1 > 0 {
            cnt1.push(c1);
            c1 = 0;
        }
      }
    }
    if c1 > 0 { cnt1.push(c1); }

    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX, usize::MAX, usize::MAX]; cnt1.len()+1];
    dp[0][0] = 0;
    for i in 0..cnt1.len() {
        let c1 = cnt1[i];
        let c0 = if i > 0 { cnt0[i-1] } else { 0 };
        dp[i+1][0] = dp[i][0] + c1;
        if dp[i][1] == usize::MAX {
            dp[i+1][1] = dp[i][0];
        } else {
            dp[i+1][1] = (dp[i][1] + c0).min(dp[i][0]);
        }
        if dp[i][1] != usize::MAX && dp[i][2] == usize::MAX {
            dp[i+1][2] = dp[i][1] + c1;
        } else if dp[1][1] != usize::MAX && dp[i][2] != usize::MAX {
            dp[i+1][2] = (dp[i][1] + c1).min(dp[i][2] + c1);
        }
    }
    let n = cnt1.len();
    println!("{}", dp[n][0].min(dp[n][1]).min(dp[n][2]));
  }
}