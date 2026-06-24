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

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
    }
    /**
    let ans = k.pow(n as u32);
    let ans = m.pow(ans as u32);
    println!("ans: {}", ans);
    */

    const MOD: usize = 998244353;
    // m^(k^n)を求める
    if m%MOD == 0 { println!("0");return; }
    // フェルマーの小定理: m^(p-1) = 1 (mod p)
    // k^n = q(p-1)+r => m^(k^n)=m(q(p-1)+r)=1*m^r (mod p)
    let mut r = 1;
    let mut base = k%(MOD-1);
    let mut exp = n;
    // r=k^n (mod MOD-1)
    while exp > 0 {
        if exp % 2 != 0 {
            r *= base;
            r %= MOD-1;
        }
        exp >>= 1;
        base *= base;
        base %= MOD-1;
    }
    // m^r
    let mut exp = r;
    let mut base = m%MOD;
    let mut ans = 1;
    while exp > 0 {
        if exp % 2 != 0 {
            ans *= base;
            ans %= MOD;
        }
        exp >>= 1;
        base *= base;
        base %= MOD;
    }

    // println!("{}", ans);
    println!("{}", modint(m, modint(k, n, MOD-1), MOD));
}