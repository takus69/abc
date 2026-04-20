use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let r#mod = 998244353;
    let mut ans = 0;
    let mut cumsum_a: Vec<usize> = vec![0];
    let mut sum_a = 0;
    let mut sum_b = 0;
    for &ai in &a {
        sum_a += ai;
        sum_a %= r#mod;
        cumsum_a.push(sum_a);
    }
    for &bj in &b {
        sum_b += bj;
        sum_b %= r#mod;
    }
    for i in 0..n {
        let mut tmp = a[i] % r#mod;
        tmp *= (i+1) % r#mod;
        tmp %= r#mod;
        tmp *= sum_b % r#mod;
        tmp %= r#mod;
        ans += tmp;
        // ans += (a[i] * (i+1) * sum_b) % r#mod;
        sum_a += a[i];
        sum_a %= r#mod;
    }
    let mut minus = 0;
    for j in 1..=m {
        for k in 1..=(n/j) {
            let mut tmp = k%r#mod;
            tmp *= (cumsum_a[((k+1)*j-1).min(n)] + r#mod - cumsum_a[k*j-1]) % r#mod;
            tmp %= r#mod;
            tmp *= b[j-1] % r#mod;
            tmp %= r#mod;
            tmp *= j % r#mod;
            tmp %= r#mod;
            // minus += ((k%r#mod) * ((cumsum_a[((k+1)*j-1).min(n)] - cumsum_a[k*j-1])%r#mod) * (b[j-1]%r#mod) * (j%r#mod)) % r#mod;
            minus += tmp;
            minus %= r#mod;
        }
    }
    ans = (ans + r#mod - minus) % r#mod;
    println!("{}", ans);
}