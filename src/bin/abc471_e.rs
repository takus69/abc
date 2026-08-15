use proconio::input;

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

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // n-1Ck-1*(a^2+a*(sum-a)*(k-1)/(n-1))
    let sum: usize = a.iter().sum();
    let mut comb = 1;
    for i in 1..k {
        // println!("n-i: {}, i: {}", n-i, i);
        comb *= n-i;
        comb %= MOD;
        comb *= modinv(i, MOD);
        comb %= MOD;
    }
    // println!("sum: {}, comb: {}", sum, comb);
    let mut ans = 0;
    for &ai in &a {
        let mut tmp1 = ai*ai;
        tmp1 %= MOD;
        tmp1 *= comb;
        tmp1 %= MOD;

        let mut tmp2 = ai*((sum-ai)%MOD);
        tmp2 %= MOD;
        tmp2 *= k-1;
        tmp2 %= MOD;
        tmp2 *= modinv(n-1, MOD);
        tmp2 %= MOD;
        tmp2 *= comb;
        tmp2 %= MOD;

        ans += tmp1 + tmp2;
        // println!("ans: {}, tmp1: {}, tmp2: {}, tmp1+tmp2: {}", ans, tmp1, tmp2, tmp1+tmp2);
        ans %= MOD;
    }
    println!("{}", ans);
}