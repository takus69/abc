use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp: Vec<usize> = vec![0, a[0]];
    for i in 2..n {
        let t = (dp[i-2]+b[i-2]).min(dp[i-1]+a[i-1]);
        dp.push(t);
    }
    let mut ans: Vec<usize> = vec![n];
    let mut now = n-1;
    while now > 0 {
        if dp[now]==dp[now-1]+a[now-1] {
            now -= 1;
            ans.push(now+1);
        } else {
            now -= 2;
            ans.push(now+1);
        }
    }
    ans.reverse();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}