use proconio::input;

const MOD: usize = 998244353;

pub fn modint(x: usize, n: usize, r#mod: usize) -> usize {
    // modを取りながら繰り返し二乗法
    let mut ret = 1;
    let mut x = x%r#mod;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret *= x;
            ret %= r#mod;
        }
        x *= x;
        x %= r#mod;
        n >>= 1;
    }
    ret
}

/// 逆元
pub fn modinv(x: usize, r#mod: usize) -> usize {
    modint(x, r#mod-2, r#mod)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut sum = 0;
    let mut sum2 = 0;
    let mut first: Vec<usize> = vec![sum];
    let mut first2: Vec<usize> = vec![sum2];
    for (i, ai) in a.iter().enumerate() {
        sum += ai;
        sum %= MOD;
        sum2 += ai*(i+1);
        sum2 %= MOD;
        first.push(sum);
        first2.push(sum2);
    }
    // let mut sum = 0;
    // let mut first2: Vec<usize> = vec![sum];
    // for &v in first.iter().skip(1) {
    //     sum += v;
    //     sum %= MOD;
    //     first2.push(sum);
    // }
    let mut sum = 0;
    let mut sum2 = 0;
    let mut last: Vec<usize> = vec![sum];
    let mut last2: Vec<usize> = vec![sum2];
    for (i, ai) in a.iter().rev().enumerate() {
        sum += ai;
        sum %= MOD;
        sum2 += ai*(i+1);
        sum2 %= MOD;
        last.push(sum);
        last2.push(sum2);
    }
    // let mut sum = 0;
    // let mut last2: Vec<usize> = vec![sum];
    // for &v in last.iter().skip(1) {
    //     sum += v;
    //     sum %= MOD;
    //     last2.push(sum);
    // }
    let mut ans = 0;
    for d in 1..=n {
        let mut tmp = 0;
        let d2 = d-1;
        if 2*d2 <= n {
            tmp += first2[d2];
            tmp += last2[d2];
            tmp += (d*(MOD+first[n-d2]-first[d2]))%MOD;
            tmp %= MOD;
        } else {
            let d3 = n-d;
            tmp += first2[d3];
            tmp += last2[d3];
            tmp += ((d3+1)*(MOD+first[n-d3]-first[d3]))%MOD;
            tmp %= MOD;
        }
        tmp *= modinv(d, MOD);
        tmp %= MOD;
        ans += tmp;
        ans %= MOD;
    }
    println!("{}", ans);
}