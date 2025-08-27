use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp: Vec<usize> = vec![0];
    for i in 2..=n {
        if i == 2 {
            dp.push(a[i-2]);
            continue;
        }
        let t = (dp[i-3]+b[i-3]).min(dp[i-2]+a[i-2]);
        dp.push(t);
    }
    println!("{}", dp.last().unwrap());
}