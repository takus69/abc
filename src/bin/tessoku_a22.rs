use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }
    let mut dp: Vec<isize> = vec![isize::MIN; n+1];
    dp[1] = 0;
    for i in 1..n {
        if dp[i] == isize::MIN { continue; }
        let ai = a[i-1];
        let bi = b[i-1];
        dp[ai] = dp[ai].max(dp[i]+100);
        dp[bi] = dp[bi].max(dp[i]+150);
    }

    println!("{}", dp[n]);
}