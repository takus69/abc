use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const MOD: usize = 998244353;
    let mut dp: Vec<usize> = vec![0; 10];
    dp[a[0]] = 1;

    for i in 0..(n-1) {
        let mut next: Vec<usize> = vec![0; 10];
        for j in 0..10 {
            let cnt = dp[j];
            let ai = a[i+1];
            let k1 = (j+ai)%10;
            let k2 = (j*ai)%10;
            next[k1] += cnt;
            next[k1] %= MOD;
            next[k2] += cnt;
            next[k2] %= MOD;
        }
        dp = next;
    }
    for k in 0..10 {
        println!("{}", dp[k]);
    }
}