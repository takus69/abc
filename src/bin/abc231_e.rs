use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        a: [usize; n],
    }

    let mut dp0 = 0;
    let mut dp1 = 1<<60;
    for i in 0..(n-1) {
        let a0 = a[i];
        let a1 = a[i+1];
        let x0 = x%a1;
        let pay0 = x0/a0;  // 繰上りなし
        let pay1 = (a1-x0)/a0;  // 繰上りあり
        let next_dp0 = (dp0+pay0).min(dp1+pay0+1);
        let next_dp1 = (dp0+pay1).min(dp1+pay1-1);
        x -= x0;
        dp0 = next_dp0;
        dp1 = next_dp1;
    }
    dp0 += x/a[n-1];
    dp1 += x/a[n-1]+1;

    println!("{}", dp0.min(dp1));
}